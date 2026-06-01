#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use super::handle_anchor_click;
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use or_poisoned::OrPoisoned;
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use crate::hooks::use_navigate;
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use leptos::ev;
use super::{LocationChange, LocationProvider, Url};
use crate::params::ParamsMap;
use core::fmt;
use futures::channel::oneshot;
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use js_sys::{try_iter, Array, JsString};
use leptos::prelude::*;
use reactive_graph::signal::ArcRwSignal;
use std::{
    borrow::Cow,
    string::String,
    sync::{Arc, Mutex},
};
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use tachys::dom::{document, window};
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use wasm_bindgen::{JsCast, JsValue};
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use web_sys::UrlSearchParams;

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
type JsValue = String;

#[derive(Clone)]
pub struct BrowserUrl {
    url: ArcRwSignal<Url>,
    pub(crate) pending_navigation: Arc<Mutex<Option<oneshot::Sender<()>>>>,
    pub(crate) path_stack: ArcStoredValue<Vec<Url>>,
    pub(crate) is_back: ArcRwSignal<bool>,
}

impl fmt::Debug for BrowserUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BrowserUrl").finish_non_exhaustive()
    }
}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
impl BrowserUrl {
    fn scroll_to_el(loc_scroll: bool) {
        if let Ok(hash) = window().location().hash() {
            if !hash.is_empty() {
                let hash = js_sys::decode_uri(&hash[1..])
                    .ok()
                    .and_then(|decoded| decoded.as_string())
                    .unwrap_or(hash);
                let el = document().get_element_by_id(&hash);
                if let Some(el) = el {
                    el.scroll_into_view();
                    return;
                }
            }
        }

        // scroll to top
        if loc_scroll {
            window().scroll_to_with_x_and_y(0.0, 0.0);
        }
    }
}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
impl LocationProvider for BrowserUrl {
    type Error = JsValue;

    fn new() -> Result<Self, JsValue> {
        let url = ArcRwSignal::new(Self::current()?);
        let path_stack = ArcStoredValue::new(
            Self::current().map(|n| vec![n]).unwrap_or_default(),
        );
        Ok(Self {
            url,
            pending_navigation: Default::default(),
            path_stack,
            is_back: Default::default(),
        })
    }

    fn as_url(&self) -> &ArcRwSignal<Url> {
        &self.url
    }

    fn current() -> Result<Url, Self::Error> {
        let location = window().location();
        Ok(Url {
            origin: location.origin()?,
            path: location.pathname()?,
            search: location
                .search()?
                .strip_prefix('?')
                .map(String::from)
                .unwrap_or_default(),
            search_params: search_params_from_web_url(
                &UrlSearchParams::new_with_str(&location.search()?)?,
            )?,
            hash: location.hash()?,
        })
    }

    fn parse(url: &str) -> Result<Url, Self::Error> {
        let base = window().location().origin()?;
        Self::parse_with_base(url, &base)
    }

    fn parse_with_base(url: &str, base: &str) -> Result<Url, Self::Error> {
        let location = web_sys::Url::new_with_base(url, base)?;
        Ok(Url {
            origin: location.origin(),
            path: location.pathname(),
            search: location
                .search()
                .strip_prefix('?')
                .map(String::from)
                .unwrap_or_default(),
            search_params: search_params_from_web_url(
                &location.search_params(),
            )?,
            hash: location.hash(),
        })
    }

    fn init(&self, base: Option<Cow<'static, str>>) {
        let navigate = {
            let url = self.url.clone();
            let pending = Arc::clone(&self.pending_navigation);
            let this = self.clone();
            move |new_url: Url, loc| {
                let same_path = {
                    let curr = url.read_untracked();
                    curr.origin() == new_url.origin()
                        && curr.path() == new_url.path()
                };

                url.set(new_url.clone());
                if same_path {
                    this.complete_navigation(&loc);
                }
                let pending = Arc::clone(&pending);
                let (tx, rx) = oneshot::channel::<()>();
                if !same_path {
                    *pending.lock().or_poisoned() = Some(tx);
                }
                let url = url.clone();
                let this = this.clone();
                async move {
                    if !same_path {
                        // if it has been canceled, ignore
                        // otherwise, complete navigation -- i.e., set URL in address bar
                        if rx.await.is_ok() {
                            // only update the URL in the browser if this is still the current URL
                            // if we've navigated to another page in the meantime, don't update the
                            // browser URL
                            let curr = url.read_untracked();
                            if curr == new_url {
                                this.complete_navigation(&loc);
                            }
                        }
                    }
                }
            }
        };

        let handle_anchor_click =
            handle_anchor_click(base, Self::parse_with_base, navigate);

        let click_handle = window_event_listener(ev::click, move |ev| {
            if let Err(e) = handle_anchor_click(ev) {
                #[cfg(feature = "tracing")]
                tracing::error!("{e:?}");
                #[cfg(not(feature = "tracing"))]
                web_sys::console::error_1(&e);
            }
        });

        // handle popstate event (forward/back navigation)
        let popstate_cb = {
            let url = self.url.clone();
            let path_stack = self.path_stack.clone();
            let is_back = self.is_back.clone();
            move || match Self::current() {
                Ok(new_url) => {
                    let mut stack = path_stack.write_value();
                    let is_navigating_back = stack.len() == 1
                        || (stack.len() >= 2
                            && stack.get(stack.len() - 2) == Some(&new_url));

                    if is_navigating_back {
                        stack.pop();
                    }

                    is_back.set(is_navigating_back);

                    url.set(new_url);
                }
                Err(e) => {
                    #[cfg(feature = "tracing")]
                    tracing::error!("{e:?}");
                    #[cfg(not(feature = "tracing"))]
                    web_sys::console::error_1(&e);
                }
            }
        };

        let popstate_handle =
            window_event_listener(ev::popstate, move |_| popstate_cb());

        on_cleanup(|| {
            click_handle.remove();
            popstate_handle.remove();
        });
    }

    fn ready_to_complete(&self) {
        if let Some(tx) = self.pending_navigation.lock().or_poisoned().take() {
            _ = tx.send(());
        }
    }

    fn complete_navigation(&self, loc: &LocationChange) {
        let history = window().history().unwrap();

        let current_path = self
            .path_stack
            .read_value()
            .last()
            .map(|url| url.to_full_path());
        let add_to_stack = current_path.as_ref() != Some(&loc.value);

        if loc.replace {
            history
                .replace_state_with_url(
                    &loc.state.to_js_value(),
                    "",
                    Some(&loc.value),
                )
                .unwrap();
        } else if add_to_stack {
            // push the "forward direction" marker
            let state = &loc.state.to_js_value();
            history
                .push_state_with_url(state, "", Some(&loc.value))
                .unwrap();
        }

        // add this URL to the "path stack" for detecting back navigations, and
        // unset "navigating back" state
        if let Ok(url) = Self::current() {
            if add_to_stack {
                self.path_stack.write_value().push(url);
            }
            self.is_back.set(false);
        }

        // scroll to el
        Self::scroll_to_el(loc.scroll);
    }

    fn redirect(loc: &str) {
        let navigate = use_navigate();
        let Some(url) = resolve_redirect_url(loc) else {
            return; // resolve_redirect_url() already logs an error
        };
        let current_origin = location().origin().unwrap();
        if url.origin() == current_origin {
            let navigate = navigate.clone();
            // delay by a tick here, so that the Action updates *before* the redirect
            request_animation_frame(move || {
                navigate(&url.href(), Default::default());
            });
            // Use set_href() if the conditions for client-side navigation were not satisfied
        } else if let Err(e) = location().set_href(&url.href()) {
            leptos::logging::error!("Failed to redirect: {e:#?}");
        }
    }

    fn is_back(&self) -> ReadSignal<bool> {
        self.is_back.read_only().into()
    }
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
impl BrowserUrl {
    pub fn parse(url: &str) -> Result<Url, JsValue> {
        let mut origin = String::new();
        let mut path = url.to_string();
        let mut search = String::new();
        let mut hash = String::new();
        if let Some(h_idx) = path.find('#') {
            hash = path[h_idx + 1..].to_string();
            path = path[..h_idx].to_string();
        }
        if let Some(q_idx) = path.find('?') {
            search = path[q_idx + 1..].to_string();
            path = path[..q_idx].to_string();
        }
        if path.starts_with("http://") || path.starts_with("https://") {
            let rest = if path.starts_with("http://") {
                &path[7..]
            } else {
                &path[8..]
            };
            if let Some(slash_idx) = rest.find('/') {
                origin =
                    path[..if path.starts_with("http://") { 7 } else { 8 }
                        + slash_idx]
                        .to_string();
                path = rest[slash_idx..].to_string();
            } else {
                origin = path.clone();
                path = "/".to_string();
            }
        }
        Ok(Url {
            origin,
            path,
            search,
            search_params: ParamsMap::default(),
            hash,
        })
    }

    pub fn redirect(_loc: &str) {}
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
impl LocationProvider for BrowserUrl {
    type Error = JsValue;

    fn new() -> Result<Self, Self::Error> {
        let url = ArcRwSignal::new(Url::default());
        let path_stack = ArcStoredValue::new(vec![]);
        Ok(Self {
            url,
            pending_navigation: Default::default(),
            path_stack,
            is_back: Default::default(),
        })
    }

    fn as_url(&self) -> &ArcRwSignal<Url> {
        &self.url
    }

    fn current() -> Result<Url, Self::Error> {
        Ok(Url::default())
    }

    fn parse(url: &str) -> Result<Url, Self::Error> {
        Self::parse(url)
    }

    fn parse_with_base(url: &str, _base: &str) -> Result<Url, Self::Error> {
        Self::parse(url)
    }

    fn init(&self, _base: Option<Cow<'static, str>>) {}

    fn ready_to_complete(&self) {}

    fn complete_navigation(&self, _loc: &LocationChange) {}

    fn redirect(loc: &str) {
        Self::redirect(loc)
    }

    fn is_back(&self) -> ReadSignal<bool> {
        self.is_back.read_only().into()
    }
}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
fn search_params_from_web_url(
    params: &UrlSearchParams,
) -> Result<ParamsMap, JsValue> {
    try_iter(params)?
        .into_iter()
        .flatten()
        .map(|pair| {
            pair.and_then(|pair| {
                let row = pair.dyn_into::<Array>()?;
                Ok((
                    String::from(row.get(0).dyn_into::<JsString>()?),
                    String::from(row.get(1).dyn_into::<JsString>()?),
                ))
            })
        })
        .collect()
}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub(crate) fn resolve_redirect_url(loc: &str) -> Option<web_sys::Url> {
    let origin = match window().location().origin() {
        Ok(origin) => origin,
        Err(e) => {
            leptos::logging::error!("Failed to get origin: {:#?}", e);
            return None;
        }
    };

    let base = origin;

    match web_sys::Url::new_with_base(loc, &base) {
        Ok(url) => Some(url),
        Err(e) => {
            leptos::logging::error!(
                "Invalid redirect location: {}",
                e.as_string().unwrap_or_default(),
            );
            None
        }
    }
}
