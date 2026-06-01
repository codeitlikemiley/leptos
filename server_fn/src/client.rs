use crate::{request::ClientReq, response::ClientRes};
use bytes::Bytes;
use futures::{Sink, Stream};
use std::{future::Future, sync::OnceLock};

static ROOT_URL: OnceLock<&'static str> = OnceLock::new();

/// Set the root server URL that all server function paths are relative to for the client.
///
/// If this is not set, it defaults to the origin.
pub fn set_server_url(url: &'static str) {
    ROOT_URL.set(url).unwrap();
}

/// Returns the root server URL for all server functions.
pub fn get_server_url() -> &'static str {
    ROOT_URL.get().copied().unwrap_or("")
}

/// A client defines a pair of request/response types and the logic to send
/// and receive them.
///
/// This trait is implemented for things like a browser `fetch` request or for
/// the `reqwest` trait. It should almost never be necessary to implement it
/// yourself, unless you’re trying to use an alternative HTTP crate on the client side.
pub trait Client<Error, InputStreamError = Error, OutputStreamError = Error> {
    /// The type of a request sent by this client.
    type Request: ClientReq<Error> + Send + 'static;
    /// The type of a response received by this client.
    type Response: ClientRes<Error> + Send + 'static;

    /// Sends the request and receives a response.
    fn send(
        req: Self::Request,
    ) -> impl Future<Output = Result<Self::Response, Error>> + Send;

    /// Opens a websocket connection to the server.
    #[allow(clippy::type_complexity)]
    fn open_websocket(
        path: &str,
    ) -> impl Future<
        Output = Result<
            (
                impl Stream<Item = Result<Bytes, Bytes>> + Send + 'static,
                impl Sink<Bytes> + Send + 'static,
            ),
            Error,
        >,
    > + Send;

    /// Spawn a future that runs in the background.
    fn spawn(future: impl Future<Output = ()> + Send + 'static);
}

#[cfg(feature = "browser")]
/// Implements [`Client`] for a `fetch` request in the browser.
pub mod browser {
    use super::{get_server_url, Client};
    use crate::{
        error::{FromServerFnError, IntoAppError, ServerFnErrorErr},
        request::browser::{BrowserRequest, RequestInner},
        response::browser::BrowserResponse,
    };
    use bytes::Bytes;
    use futures::{Sink, SinkExt, StreamExt};
    use gloo_net::websocket::{Message, WebSocketError};
    use send_wrapper::SendWrapper;
    use std::future::Future;

    /// Implements [`Client`] for a `fetch` request in the browser.
    pub struct BrowserClient;

    impl<
            Error: FromServerFnError,
            InputStreamError: FromServerFnError,
            OutputStreamError: FromServerFnError,
        > Client<Error, InputStreamError, OutputStreamError> for BrowserClient
    {
        type Request = BrowserRequest;
        type Response = BrowserResponse;

        fn send(
            req: Self::Request,
        ) -> impl Future<Output = Result<Self::Response, Error>> + Send
        {
            SendWrapper::new(async move {
                let req = req.0.take();
                let RequestInner {
                    request,
                    mut abort_ctrl,
                } = req;
                let res = request
                    .send()
                    .await
                    .map(|res| BrowserResponse(SendWrapper::new(res)))
                    .map_err(|e| {
                        ServerFnErrorErr::Request(e.to_string())
                            .into_app_error()
                    });

                // at this point, the future has successfully resolved without being dropped, so we
                // can prevent the `AbortController` from firing
                if let Some(ctrl) = abort_ctrl.as_mut() {
                    ctrl.prevent_cancellation();
                }
                res
            })
        }

        fn open_websocket(
            url: &str,
        ) -> impl Future<
            Output = Result<
                (
                    impl futures::Stream<Item = Result<Bytes, Bytes>>
                        + Send
                        + 'static,
                    impl futures::Sink<Bytes> + Send + 'static,
                ),
                Error,
            >,
        > + Send {
            let mut websocket_server_url = get_server_url().to_string();
            if let Some(postfix) = websocket_server_url.strip_prefix("http://")
            {
                websocket_server_url = format!("ws://{postfix}");
            } else if let Some(postfix) =
                websocket_server_url.strip_prefix("https://")
            {
                websocket_server_url = format!("wss://{postfix}");
            }
            let url = format!("{websocket_server_url}{url}");
            SendWrapper::new(async move {
                let websocket =
                    gloo_net::websocket::futures::WebSocket::open(&url)
                        .map_err(|err| {
                            web_sys::console::error_1(&err.to_string().into());
                            Error::from_server_fn_error(
                                ServerFnErrorErr::Request(err.to_string()),
                            )
                        })?;
                let (sink, stream) = websocket.split();

                let stream = stream.map(|message| match message {
                    Ok(message) => Ok(match message {
                        Message::Text(text) => Bytes::from(text),
                        Message::Bytes(bytes) => Bytes::from(bytes),
                    }),
                    Err(err) => {
                        web_sys::console::error_1(&err.to_string().into());
                        Err(OutputStreamError::from_server_fn_error(
                            ServerFnErrorErr::Request(err.to_string()),
                        )
                        .ser())
                    }
                });
                let stream = SendWrapper::new(stream);

                struct SendWrapperSink<S> {
                    sink: SendWrapper<S>,
                }

                impl<S> SendWrapperSink<S> {
                    fn new(sink: S) -> Self {
                        Self {
                            sink: SendWrapper::new(sink),
                        }
                    }
                }

                impl<S, Item> Sink<Item> for SendWrapperSink<S>
                where
                    S: Sink<Item> + Unpin,
                {
                    type Error = S::Error;

                    fn poll_ready(
                        self: std::pin::Pin<&mut Self>,
                        cx: &mut std::task::Context<'_>,
                    ) -> std::task::Poll<Result<(), Self::Error>>
                    {
                        self.get_mut().sink.poll_ready_unpin(cx)
                    }

                    fn start_send(
                        self: std::pin::Pin<&mut Self>,
                        item: Item,
                    ) -> Result<(), Self::Error> {
                        self.get_mut().sink.start_send_unpin(item)
                    }

                    fn poll_flush(
                        self: std::pin::Pin<&mut Self>,
                        cx: &mut std::task::Context<'_>,
                    ) -> std::task::Poll<Result<(), Self::Error>>
                    {
                        self.get_mut().sink.poll_flush_unpin(cx)
                    }

                    fn poll_close(
                        self: std::pin::Pin<&mut Self>,
                        cx: &mut std::task::Context<'_>,
                    ) -> std::task::Poll<Result<(), Self::Error>>
                    {
                        self.get_mut().sink.poll_close_unpin(cx)
                    }
                }

                let sink = sink.with(|message: Bytes| async move {
                    Ok::<Message, WebSocketError>(Message::Bytes(
                        message.into(),
                    ))
                });
                let sink = SendWrapperSink::new(Box::pin(sink));

                Ok((stream, sink))
            })
        }

        fn spawn(future: impl Future<Output = ()> + Send + 'static) {
            wasm_bindgen_futures::spawn_local(future);
        }
    }
}

#[cfg(feature = "reqwest")]
/// Implements [`Client`] for a request made by [`reqwest`].
pub mod reqwest {
    use super::{get_server_url, Client};
    use crate::{
        error::{FromServerFnError, IntoAppError, ServerFnErrorErr},
        request::reqwest::CLIENT,
    };
    use bytes::Bytes;
    use futures::{SinkExt, StreamExt, TryFutureExt};
    use reqwest::{Request, Response};
    use std::future::Future;

    /// Implements [`Client`] for a request made by [`reqwest`].
    pub struct ReqwestClient;

    impl<
            Error: FromServerFnError,
            InputStreamError: FromServerFnError,
            OutputStreamError: FromServerFnError,
        > Client<Error, InputStreamError, OutputStreamError> for ReqwestClient
    {
        type Request = Request;
        type Response = Response;

        fn send(
            req: Self::Request,
        ) -> impl Future<Output = Result<Self::Response, Error>> + Send
        {
            CLIENT.execute(req).map_err(|e| {
                ServerFnErrorErr::Request(e.to_string()).into_app_error()
            })
        }

        async fn open_websocket(
            path: &str,
        ) -> Result<
            (
                impl futures::Stream<Item = Result<Bytes, Bytes>> + Send + 'static,
                impl futures::Sink<Bytes> + Send + 'static,
            ),
            Error,
        > {
            let mut websocket_server_url = get_server_url().to_string();
            if let Some(postfix) = websocket_server_url.strip_prefix("http://")
            {
                websocket_server_url = format!("ws://{postfix}");
            } else if let Some(postfix) =
                websocket_server_url.strip_prefix("https://")
            {
                websocket_server_url = format!("wss://{postfix}");
            }
            let url = format!("{websocket_server_url}{path}");
            let (ws_stream, _) =
                tokio_tungstenite::connect_async(url).await.map_err(|e| {
                    Error::from_server_fn_error(ServerFnErrorErr::Request(
                        e.to_string(),
                    ))
                })?;

            let (write, read) = ws_stream.split();

            Ok((
                read.map(|msg| match msg {
                    Ok(msg) => Ok(msg.into_data()),
                    Err(e) => Err(OutputStreamError::from_server_fn_error(
                        ServerFnErrorErr::Request(e.to_string()),
                    )
                    .ser()),
                }),
                write.with(|msg: Bytes| async move {
                    Ok::<
                        tokio_tungstenite::tungstenite::Message,
                        tokio_tungstenite::tungstenite::Error,
                    >(
                        tokio_tungstenite::tungstenite::Message::Binary(msg)
                    )
                }),
            ))
        }

        fn spawn(future: impl Future<Output = ()> + Send + 'static) {
            tokio::spawn(future);
        }
    }
}

#[cfg(not(feature = "browser"))]
/// Implements [`Client`] stubs for when browser feature is disabled.
pub mod browser {
    use super::Client;
    use crate::error::FromServerFnError;
    use bytes::Bytes;
    use std::future::Future;

    /// Implements [`Client`] for a `fetch` request in the browser.
    pub struct BrowserClient;

    /// Stub request type.
    pub struct BrowserRequest;
    /// Stub response type.
    pub struct BrowserResponse;

    impl<
            Error: FromServerFnError,
            InputStreamError: FromServerFnError,
            OutputStreamError: FromServerFnError,
        > Client<Error, InputStreamError, OutputStreamError> for BrowserClient
    {
        type Request = BrowserRequest;
        type Response = BrowserResponse;

        async fn send(_req: Self::Request) -> Result<Self::Response, Error> {
            unreachable!()
        }

        async fn open_websocket(
            _url: &str,
        ) -> Result<
            (
                impl futures::Stream<Item = Result<Bytes, Bytes>> + Send + 'static,
                impl futures::Sink<Bytes> + Send + 'static,
            ),
            Error,
        > {
            let stream = futures::stream::empty::<Result<Bytes, Bytes>>();
            let sink = futures::sink::drain();
            Ok((stream, sink))
        }

        fn spawn(_future: impl Future<Output = ()> + Send + 'static) {
            unreachable!()
        }
    }

    impl<E> crate::request::ClientReq<E> for BrowserRequest {
        type FormData = ();

        fn try_new_req_query(
            _path: &str,
            _content_type: &str,
            _accepts: &str,
            _query: &str,
            _method: http::Method,
        ) -> Result<Self, E> {
            unreachable!()
        }

        fn try_new_req_text(
            _path: &str,
            _content_type: &str,
            _accepts: &str,
            _body: String,
            _method: http::Method,
        ) -> Result<Self, E> {
            unreachable!()
        }

        fn try_new_req_bytes(
            _path: &str,
            _content_type: &str,
            _accepts: &str,
            _body: bytes::Bytes,
            _method: http::Method,
        ) -> Result<Self, E> {
            unreachable!()
        }

        fn try_new_req_form_data(
            _path: &str,
            _accepts: &str,
            _content_type: &str,
            _body: Self::FormData,
            _method: http::Method,
        ) -> Result<Self, E> {
            unreachable!()
        }

        fn try_new_req_multipart(
            _path: &str,
            _accepts: &str,
            _body: Self::FormData,
            _method: http::Method,
        ) -> Result<Self, E> {
            unreachable!()
        }

        fn try_new_req_streaming(
            _path: &str,
            _accepts: &str,
            _content_type: &str,
            _body: impl futures::Stream<Item = bytes::Bytes> + Send + 'static,
            _method: http::Method,
        ) -> Result<Self, E> {
            unreachable!()
        }
    }

    impl<E> crate::response::ClientRes<E> for BrowserResponse {
        fn try_into_string(
            self,
        ) -> impl std::future::Future<Output = Result<String, E>> + Send
        {
            async { unreachable!() }
        }

        fn try_into_bytes(
            self,
        ) -> impl std::future::Future<Output = Result<bytes::Bytes, E>> + Send
        {
            async { unreachable!() }
        }

        fn try_into_stream(
            self,
        ) -> Result<
            impl futures::Stream<Item = Result<bytes::Bytes, bytes::Bytes>>
                + Send
                + Sync
                + 'static,
            E,
        > {
            Ok(futures::stream::empty())
        }

        fn status(&self) -> u16 {
            unreachable!()
        }

        fn status_text(&self) -> String {
            unreachable!()
        }

        fn location(&self) -> String {
            unreachable!()
        }

        fn has_redirect(&self) -> bool {
            unreachable!()
        }
    }
}
