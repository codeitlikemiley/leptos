#![allow(missing_docs)]
#![allow(unused_variables)]

use super::{CastFrom, RemoveEventHandler};
use crate::view::{Mountable, ToTemplate};
use std::borrow::Cow;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Dom;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Node;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Text;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Element;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Placeholder;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Event;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClassList;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CssStyleDeclaration;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TemplateElement;

impl AsRef<Node> for Element {
    fn as_ref(&self) -> &Node {
        static NODE: Node = Node;
        &NODE
    }
}
impl AsRef<Node> for Text {
    fn as_ref(&self) -> &Node {
        static NODE: Node = Node;
        &NODE
    }
}
impl AsRef<Node> for Placeholder {
    fn as_ref(&self) -> &Node {
        static NODE: Node = Node;
        &NODE
    }
}
impl AsRef<Node> for Node {
    fn as_ref(&self) -> &Node {
        self
    }
}

impl Dom {
    pub fn intern(text: &str) -> &str {
        text
    }

    pub fn create_element(tag: &str, namespace: Option<&str>) -> Element {
        Element
    }

    pub fn create_text_node(text: &str) -> Text {
        Text
    }

    pub fn create_placeholder() -> Placeholder {
        Placeholder
    }

    pub fn set_text(node: &Text, text: &str) {}

    pub fn set_attribute(node: &Element, name: &str, value: &str) {}

    pub fn remove_attribute(node: &Element, name: &str) {}

    pub fn insert_node(parent: &Element, new_child: &Node, anchor: Option<&Node>) {}

    pub fn try_insert_node(parent: &Element, new_child: &Node, anchor: Option<&Node>) -> bool {
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
    {}

    pub fn try_mount_before<M>(new_child: &mut M, before: &Node) -> bool
    where
        M: Mountable,
    {
        true
    }

    pub fn set_property_or_value(el: &Element, key: &str, value: &wasm_bindgen::JsValue) {}

    pub fn set_property(el: &Element, key: &str, value: &wasm_bindgen::JsValue) {}

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
        T::cast_from(Element).unwrap()
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
        ClassList
    }

    pub fn add_class(list: &ClassList, name: &str) {}

    pub fn remove_class(list: &ClassList, name: &str) {}

    pub fn style(el: &Element) -> CssStyleDeclaration {
        CssStyleDeclaration
    }

    pub fn set_css_property(
        style: &CssStyleDeclaration,
        name: &str,
        value: &str,
    ) {}

    pub fn remove_css_property(style: &CssStyleDeclaration, name: &str) {}

    pub fn set_inner_html(el: &Element, html: &str) {}

    pub fn get_template<V>() -> TemplateElement
    where
        V: ToTemplate + 'static,
    {
        TemplateElement
    }

    pub fn clone_template(tpl: &TemplateElement) -> Element {
        Element
    }

    pub fn create_element_from_html(html: Cow<'static, str>) -> Element {
        Element
    }

    pub fn create_svg_element_from_html(html: Cow<'static, str>) -> Element {
        Element
    }
}

impl Mountable for Node {
    fn unmount(&mut self) {}
    fn mount(&mut self, _parent: &Element, _marker: Option<&Node>) {}
    fn try_mount(&mut self, _parent: &Element, _marker: Option<&Node>) -> bool { true }
    fn insert_before_this(&self, _child: &mut dyn Mountable) -> bool { true }
    fn elements(&self) -> Vec<Element> { vec![] }
}

impl Mountable for Text {
    fn unmount(&mut self) {}
    fn mount(&mut self, _parent: &Element, _marker: Option<&Node>) {}
    fn try_mount(&mut self, _parent: &Element, _marker: Option<&Node>) -> bool { true }
    fn insert_before_this(&self, _child: &mut dyn Mountable) -> bool { true }
    fn elements(&self) -> Vec<Element> { vec![] }
}

impl Mountable for Placeholder {
    fn unmount(&mut self) {}
    fn mount(&mut self, _parent: &Element, _marker: Option<&Node>) {}
    fn try_mount(&mut self, _parent: &Element, _marker: Option<&Node>) -> bool { true }
    fn insert_before_this(&self, _child: &mut dyn Mountable) -> bool { true }
    fn elements(&self) -> Vec<Element> { vec![] }
}

impl Mountable for Element {
    fn unmount(&mut self) {}
    fn mount(&mut self, _parent: &Element, _marker: Option<&Node>) {}
    fn try_mount(&mut self, _parent: &Element, _marker: Option<&Node>) -> bool { true }
    fn insert_before_this(&self, _child: &mut dyn Mountable) -> bool { true }
    fn elements(&self) -> Vec<Element> { vec![self.clone()] }
}

impl CastFrom<Node> for Text {
    fn cast_from(_node: Node) -> Option<Text> { Some(Text) }
}

impl CastFrom<Node> for Placeholder {
    fn cast_from(_node: Node) -> Option<Placeholder> { Some(Placeholder) }
}

impl CastFrom<Node> for Element {
    fn cast_from(_node: Node) -> Option<Element> { Some(Element) }
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
