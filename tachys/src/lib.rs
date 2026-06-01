//! Allows rendering user interfaces based on a statically-typed view tree.
//!
//! This view tree is generic over rendering backends, and agnostic about reactivity/change
//! detection.

// this is specifically used for `unsized_const_params` below
// this allows us to use const generic &'static str for static text nodes and attributes
#![allow(incomplete_features)]
#![cfg_attr(
    all(feature = "nightly", rustc_nightly),
    feature(unsized_const_params)
)]
// support for const generic &'static str has now moved back and forth between
// these two features a couple times; we'll just enable both
#![cfg_attr(all(feature = "nightly", rustc_nightly), feature(adt_const_params))]
#![deny(missing_docs)]

/// Commonly-used traits.
pub mod prelude {
    pub use crate::{
        html::{
            attribute::{
                any_attribute::IntoAnyAttribute,
                aria::AriaAttributes,
                custom::CustomAttribute,
                global::{
                    ClassAttribute, GlobalAttributes, GlobalOnAttributes,
                    OnAttribute, OnTargetAttribute, PropAttribute,
                    StyleAttribute,
                },
                IntoAttributeValue,
            },
            directive::DirectiveAttribute,
            element::{ElementChild, ElementExt, InnerHtmlAttribute},
            node_ref::NodeRefAttribute,
        },
        renderer::{dom::Dom, Renderer},
        view::{
            add_attr::AddAnyAttr,
            any_view::{AnyView, IntoAny, IntoMaybeErased},
            IntoRender, Mountable, Render, RenderHtml,
        },
    };
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
use renderer::dom::{JsValue, Node};
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use wasm_bindgen::JsValue;
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use web_sys::Node;

/// Helpers for interacting with the DOM.
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub mod dom;

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[allow(missing_docs)]
pub mod dom {
    use crate::{
        renderer::dom::JsCast,
        web_sys::{Document, HtmlElement, Window},
    };

    /// Stub document
    pub fn document() -> Document {
        Document
    }
    /// Stub body
    pub fn body() -> HtmlElement {
        HtmlElement
    }
    /// Stub window
    pub fn window() -> Window {
        Window
    }
    /// Stub event_target
    pub fn event_target<T>(event: &crate::web_sys::Event) -> T
    where
        T: JsCast,
    {
        panic!()
    }
    /// Stub event_target_value
    pub fn event_target_value<T>(event: &T) -> String {
        String::new()
    }
    /// Stub event_target_checked
    pub fn event_target_checked(ev: &crate::web_sys::Event) -> bool {
        false
    }
}
/// Types for building a statically-typed HTML view tree.
pub mod html;
/// Supports adding interactivity to HTML.
pub mod hydration;
/// Types for MathML.
pub mod mathml;
/// Defines various backends that can render views.
pub mod renderer;
/// Rendering views to HTML.
pub mod ssr;
/// Types for SVG.
pub mod svg;
/// Core logic for manipulating views.
pub mod view;

pub use either_of as either;
#[cfg(feature = "islands")]
#[doc(hidden)]
pub use wasm_bindgen;
#[cfg(feature = "islands")]
#[doc(hidden)]
pub use web_sys;

/// View implementations for the `oco_ref` crate (cheaply-cloned string types).
#[cfg(feature = "oco")]
pub mod oco;
/// View implementations for the `reactive_graph` crate.
#[cfg(feature = "reactive_graph")]
pub mod reactive_graph;

/// A type-erased container.
pub mod erased;

pub(crate) trait UnwrapOrDebug {
    type Output;

    fn or_debug(self, el: &Node, label: &'static str);

    fn ok_or_debug(
        self,
        el: &Node,
        label: &'static str,
    ) -> Option<Self::Output>;
}

impl<T> UnwrapOrDebug for Result<T, JsValue> {
    type Output = T;

    #[track_caller]
    fn or_debug(self, el: &Node, name: &'static str) {
        #[cfg(all(
            target_arch = "wasm32",
            target_os = "unknown",
            any(debug_assertions, leptos_debuginfo)
        ))]
        {
            if let Err(err) = self {
                let location = std::panic::Location::caller();
                web_sys::console::warn_3(
                    &JsValue::from_str(&format!(
                        "[WARNING] Non-fatal error at {location}, while \
                         calling {name} on "
                    )),
                    el,
                    &err,
                );
            }
        }
        #[cfg(not(all(
            target_arch = "wasm32",
            target_os = "unknown",
            any(debug_assertions, leptos_debuginfo)
        )))]
        {
            _ = el;
            _ = name;
            _ = self;
        }
    }

    #[track_caller]
    fn ok_or_debug(
        self,
        el: &Node,
        name: &'static str,
    ) -> Option<Self::Output> {
        #[cfg(all(
            target_arch = "wasm32",
            target_os = "unknown",
            any(debug_assertions, leptos_debuginfo)
        ))]
        {
            if let Err(err) = &self {
                let location = std::panic::Location::caller();
                web_sys::console::warn_3(
                    &JsValue::from_str(&format!(
                        "[WARNING] Non-fatal error at {location}, while \
                         calling {name} on "
                    )),
                    el,
                    err,
                );
            }
            self.ok()
        }
        #[cfg(not(all(
            target_arch = "wasm32",
            target_os = "unknown",
            any(debug_assertions, leptos_debuginfo)
        )))]
        {
            _ = el;
            _ = name;
            self.ok()
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! or_debug {
    ($action:expr, $el:expr, $label:literal) => {
        if cfg!(any(debug_assertions, leptos_debuginfo)) {
            $crate::UnwrapOrDebug::or_debug($action, $el, $label);
        } else {
            _ = $action;
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! ok_or_debug {
    ($action:expr, $el:expr, $label:literal) => {
        if cfg!(any(debug_assertions, leptos_debuginfo)) {
            $crate::UnwrapOrDebug::ok_or_debug($action, $el, $label)
        } else {
            $action.ok()
        }
    };
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[allow(non_camel_case_types, missing_docs)]
pub mod web_sys {
    macro_rules! mock_types {
        ($($ty:ident),* $(,)?) => {
            $(
                #[derive(Clone, Debug)]
                pub struct $ty;

                impl $crate::renderer::dom::JsCast for $ty {
                    fn unchecked_into<T>(self) -> T { panic!() }
                }
            )*
        };
    }

    mock_types! {
        Window,
        Document,
        HtmlElement,
        HtmlInputElement,
        Element,
        Event,
        Comment,
        Text,
        Node,
        HtmlTemplateElement,
        DocumentFragment,
        DomTokenList,
        CssStyleDeclaration,
        ShadowRoot,
        HtmlCollection,
        DomStringMap,
        AddEventListenerOptions,
        AnimationEvent,
        BeforeUnloadEvent,
        ClipboardEvent,
        CompositionEvent,
        CustomEvent,
        DeviceMotionEvent,
        DeviceOrientationEvent,
        DragEvent,
        ErrorEvent,
        FocusEvent,
        GamepadEvent,
        HashChangeEvent,
        InputEvent,
        KeyboardEvent,
        MessageEvent,
        MouseEvent,
        PageTransitionEvent,
        PointerEvent,
        PopStateEvent,
        ProgressEvent,
        PromiseRejectionEvent,
        SecurityPolicyViolationEvent,
        StorageEvent,
        SubmitEvent,
        TouchEvent,
        TransitionEvent,
        UiEvent,
        WheelEvent,
        HtmlHtmlElement,
        HtmlBaseElement,
        HtmlHeadElement,
        HtmlLinkElement,
        HtmlMetaElement,
        HtmlStyleElement,
        HtmlTitleElement,
        HtmlBodyElement,
        HtmlHeadingElement,
        HtmlQuoteElement,
        HtmlDivElement,
        HtmlDListElement,
        HtmlHrElement,
        HtmlLiElement,
        HtmlOListElement,
        HtmlParagraphElement,
        HtmlPreElement,
        HtmlUListElement,
        HtmlAnchorElement,
        HtmlBrElement,
        HtmlDataElement,
        HtmlSpanElement,
        HtmlTimeElement,
        HtmlAreaElement,
        HtmlAudioElement,
        HtmlImageElement,
        HtmlMapElement,
        HtmlTrackElement,
        HtmlVideoElement,
        HtmlEmbedElement,
        HtmlIFrameElement,
        HtmlObjectElement,
        HtmlParamElement,
        HtmlPictureElement,
        HtmlSourceElement,
        SvgElement,
        HtmlCanvasElement,
        HtmlScriptElement,
        HtmlModElement,
        HtmlTableCaptionElement,
        HtmlTableColElement,
        HtmlTableElement,
        HtmlTableSectionElement,
        HtmlTableCellElement,
        HtmlTableRowElement,
        HtmlButtonElement,
        HtmlDataListElement,
        HtmlFieldSetElement,
        HtmlFormElement,
        HtmlLabelElement,
        HtmlLegendElement,
        HtmlMeterElement,
        HtmlOptGroupElement,
        HtmlOutputElement,
        HtmlProgressElement,
        HtmlSelectElement,
        HtmlTextAreaElement,
        HtmlDetailsElement,
        HtmlDialogElement,
        HtmlMenuElement,
        HtmlSlotElement,
        HtmlOptionElement,
    }

    impl AddEventListenerOptions {
        pub fn new() -> Self {
            AddEventListenerOptions
        }
    }

    static DUMMY_ELEMENT: Element = Element;

    impl Document {
        pub fn body(&self) -> Option<HtmlElement> {
            Some(HtmlElement)
        }
        pub fn document_element(&self) -> Option<Element> {
            Some(Element)
        }
        pub fn head(&self) -> Option<HtmlHeadElement> {
            Some(HtmlHeadElement)
        }
        pub fn create_element(
            &self,
            _tag: &str,
        ) -> Result<Element, crate::renderer::dom::JsValue> {
            Ok(Element)
        }
        pub fn set_title(&self, _title: &str) {}
    }

    impl Element {
        pub fn append_child(
            &self,
            _child: &Element,
        ) -> Result<Element, crate::renderer::dom::JsValue> {
            Ok(Element)
        }
    }

    impl HtmlHeadElement {
        pub fn append_child(
            &self,
            _child: &Element,
        ) -> Result<Element, crate::renderer::dom::JsValue> {
            Ok(Element)
        }
    }

    macro_rules! impl_deref_element {
        ($($t:ty),*) => {
            $(
                impl std::ops::Deref for $t {
                    type Target = Element;
                    fn deref(&self) -> &Self::Target { &DUMMY_ELEMENT }
                }

                impl AsRef<Element> for $t {
                    fn as_ref(&self) -> &Element { &DUMMY_ELEMENT }
                }
            )*
        };
    }

    impl_deref_element! {
        HtmlHtmlElement,
        HtmlBodyElement,
        HtmlHeadElement,
        HtmlTitleElement,
        HtmlMetaElement,
        HtmlLinkElement,
        HtmlStyleElement,
        HtmlScriptElement,
        HtmlElement
    }
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[allow(missing_docs)]
pub mod wasm_bindgen {
    pub use crate::renderer::dom::{JsCast, JsValue};
}
