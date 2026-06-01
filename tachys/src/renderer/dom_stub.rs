#![allow(missing_docs)]
#![allow(unused_variables)]

use super::{CastFrom, RemoveEventHandler};
use crate::view::{Mountable, ToTemplate};
use std::borrow::Cow;
use wasm_bindgen::JsValue;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Dom;

pub type Node = web_sys::Node;
pub type Text = web_sys::Text;
pub type Element = web_sys::Element;
pub type Placeholder = web_sys::Comment;
pub type Event = wasm_bindgen::JsValue;
pub type ClassList = web_sys::DomTokenList;
pub type CssStyleDeclaration = web_sys::CssStyleDeclaration;
pub type TemplateElement = web_sys::HtmlTemplateElement;

impl Dom {
    pub fn intern(text: &str) -> &str {
        text
    }

    pub fn create_element(tag: &str, namespace: Option<&str>) -> Element {
        panic!(
            "Dom::create_element is a browser-only API and cannot be called \
             on the server."
        )
    }

    pub fn create_text_node(text: &str) -> Text {
        panic!(
            "Dom::create_text_node is a browser-only API and cannot be called \
             on the server."
        )
    }

    pub fn create_placeholder() -> Placeholder {
        panic!(
            "Dom::create_placeholder is a browser-only API and cannot be \
             called on the server."
        )
    }

    pub fn set_text(node: &Text, text: &str) {}

    pub fn set_attribute(node: &Element, name: &str, value: &str) {}

    pub fn remove_attribute(node: &Element, name: &str) {}

    pub fn insert_node(
        parent: &Element,
        new_child: &Node,
        anchor: Option<&Node>,
    ) {
    }

    pub fn try_insert_node(
        parent: &Element,
        new_child: &Node,
        anchor: Option<&Node>,
    ) -> bool {
        true
    }

    pub fn remove_node(parent: &Element, child: &Node) -> Option<Node> {
        None
    }

    pub fn remove(node: &Node) {}

    pub fn get_parent(node: &Node) -> Option<Node> {
        None
    }

    pub fn first_child(node: &Node) -> Option<Node> {
        None
    }

    pub fn next_sibling(node: &Node) -> Option<Node> {
        None
    }

    pub fn log_node(node: &Node) {}

    pub fn clear_children(parent: &Element) {}

    pub fn mount_before<M>(new_child: &mut M, before: &Node)
    where
        M: Mountable,
    {
    }

    pub fn try_mount_before<M>(new_child: &mut M, before: &Node) -> bool
    where
        M: Mountable,
    {
        true
    }

    pub fn set_property_or_value(el: &Element, key: &str, value: &JsValue) {}

    pub fn set_property(el: &Element, key: &str, value: &JsValue) {}

    pub fn add_event_listener(
        el: &Element,
        name: &str,
        cb: Box<dyn FnMut(Event)>,
    ) -> RemoveEventHandler<Element> {
        RemoveEventHandler::new(|| {})
    }

    pub fn add_event_listener_use_capture(
        el: &Element,
        name: &str,
        cb: Box<dyn FnMut(Event)>,
    ) -> RemoveEventHandler<Element> {
        RemoveEventHandler::new(|| {})
    }

    pub fn event_target<T>(ev: &Event) -> T
    where
        T: CastFrom<Element>,
    {
        panic!(
            "Dom::event_target is a browser-only API and cannot be called on \
             the server."
        )
    }

    pub fn add_event_listener_delegated(
        el: &Element,
        name: Cow<'static, str>,
        delegation_key: Cow<'static, str>,
        cb: Box<dyn FnMut(Event)>,
    ) -> RemoveEventHandler<Element> {
        RemoveEventHandler::new(|| {})
    }

    pub fn class_list(el: &Element) -> ClassList {
        panic!(
            "Dom::class_list is a browser-only API and cannot be called on \
             the server."
        )
    }

    pub fn add_class(list: &ClassList, name: &str) {}

    pub fn remove_class(list: &ClassList, name: &str) {}

    pub fn style(el: &Element) -> CssStyleDeclaration {
        panic!(
            "Dom::style is a browser-only API and cannot be called on the \
             server."
        )
    }

    pub fn set_css_property(
        style: &CssStyleDeclaration,
        name: &str,
        value: &str,
    ) {
    }

    pub fn remove_css_property(style: &CssStyleDeclaration, name: &str) {}

    pub fn set_inner_html(el: &Element, html: &str) {}

    pub fn get_template<V>() -> TemplateElement
    where
        V: ToTemplate + 'static,
    {
        panic!(
            "Dom::get_template is a browser-only API and cannot be called on \
             the server."
        )
    }

    pub fn clone_template(tpl: &TemplateElement) -> Element {
        panic!(
            "Dom::clone_template is a browser-only API and cannot be called \
             on the server."
        )
    }

    pub fn create_element_from_html(html: Cow<'static, str>) -> Element {
        panic!(
            "Dom::create_element_from_html is a browser-only API and cannot \
             be called on the server."
        )
    }

    pub fn create_svg_element_from_html(html: Cow<'static, str>) -> Element {
        panic!(
            "Dom::create_svg_element_from_html is a browser-only API and \
             cannot be called on the server."
        )
    }
}

impl Mountable for Node {
    fn unmount(&mut self) {}
    fn mount(&mut self, _parent: &Element, _marker: Option<&Node>) {}
    fn try_mount(&mut self, _parent: &Element, _marker: Option<&Node>) -> bool {
        true
    }
    fn insert_before_this(&self, _child: &mut dyn Mountable) -> bool {
        true
    }
    fn elements(&self) -> Vec<Element> {
        vec![]
    }
}

impl Mountable for Text {
    fn unmount(&mut self) {}
    fn mount(&mut self, _parent: &Element, _marker: Option<&Node>) {}
    fn try_mount(&mut self, _parent: &Element, _marker: Option<&Node>) -> bool {
        true
    }
    fn insert_before_this(&self, _child: &mut dyn Mountable) -> bool {
        true
    }
    fn elements(&self) -> Vec<Element> {
        vec![]
    }
}

impl Mountable for Comment {
    fn unmount(&mut self) {}
    fn mount(&mut self, _parent: &Element, _marker: Option<&Node>) {}
    fn try_mount(&mut self, _parent: &Element, _marker: Option<&Node>) -> bool {
        true
    }
    fn insert_before_this(&self, _child: &mut dyn Mountable) -> bool {
        true
    }
    fn elements(&self) -> Vec<Element> {
        vec![]
    }
}

impl Mountable for Element {
    fn unmount(&mut self) {}
    fn mount(&mut self, _parent: &Element, _marker: Option<&Node>) {}
    fn try_mount(&mut self, _parent: &Element, _marker: Option<&Node>) -> bool {
        true
    }
    fn insert_before_this(&self, _child: &mut dyn Mountable) -> bool {
        true
    }
    fn elements(&self) -> Vec<Element> {
        vec![self.clone()]
    }
}

impl CastFrom<Node> for Text {
    fn cast_from(node: Node) -> Option<Text> {
        None
    }
}

impl CastFrom<Node> for Comment {
    fn cast_from(node: Node) -> Option<Comment> {
        None
    }
}

impl CastFrom<Node> for Element {
    fn cast_from(node: Node) -> Option<Element> {
        None
    }
}

impl<T> CastFrom<wasm_bindgen::JsValue> for T
where
    T: wasm_bindgen::JsCast,
{
    fn cast_from(source: wasm_bindgen::JsValue) -> Option<Self> {
        None
    }
}

impl<T> CastFrom<Element> for T
where
    T: wasm_bindgen::JsCast,
{
    fn cast_from(source: Element) -> Option<Self> {
        None
    }
}
