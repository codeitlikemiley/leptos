#![allow(missing_docs)]
#![allow(unused_variables)]

use super::{CastFrom, RemoveEventHandler};
use crate::view::{Mountable, ToTemplate};
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsValue;

impl JsValue {
    pub const UNDEFINED: JsValue = JsValue;
    pub fn from_str(_s: &str) -> Self { JsValue }
}

macro_rules! impl_from_jsvalue {
    ($($t:ty),*) => {
        $(
            impl From<$t> for JsValue {
                fn from(_: $t) -> Self { JsValue }
            }
            impl From<Option<$t>> for JsValue {
                fn from(_: Option<$t>) -> Self { JsValue }
            }
        )*
    };
}

impl_from_jsvalue! {
    bool,
    usize, u8, u16, u32, u64, u128,
    isize, i8, i16, i32, i64, i128,
    f32, f64,
    String
}

impl<'a> From<&'a str> for JsValue {
    fn from(_: &'a str) -> Self { JsValue }
}
impl<'a> From<Option<&'a str>> for JsValue {
    fn from(_: Option<&'a str>) -> Self { JsValue }
}

impl<'a> From<&'a String> for JsValue {
    fn from(_: &'a String) -> Self { JsValue }
}
impl<'a> From<Option<&'a String>> for JsValue {
    fn from(_: Option<&'a String>) -> Self { JsValue }
}

impl<'a> From<Cow<'a, str>> for JsValue {
    fn from(_: Cow<'a, str>) -> Self { JsValue }
}
impl<'a> From<Option<Cow<'a, str>>> for JsValue {
    fn from(_: Option<Cow<'a, str>>) -> Self { JsValue }
}

impl<'a> From<&'a Cow<'a, str>> for JsValue {
    fn from(_: &'a Cow<'a, str>) -> Self { JsValue }
}
impl<'a> From<Option<&'a Cow<'a, str>>> for JsValue {
    fn from(_: Option<&'a Cow<'a, str>>) -> Self { JsValue }
}

impl From<Option<JsValue>> for JsValue {
    fn from(_: Option<JsValue>) -> Self { JsValue }
}

pub trait JsCast {
    fn unchecked_into<T>(self) -> T;
}

impl JsCast for Element {
    fn unchecked_into<T>(self) -> T { panic!() }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Dom;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Text;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Comment;
pub type Placeholder = Comment;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Element;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassList;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CssStyleDeclaration;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateElement;

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

impl Element {
    pub fn unchecked_into<T>(self) -> T
    where
        T: JsCast,
    {
        panic!("Element::unchecked_into is a browser-only API and cannot be called on the server.")
    }

    pub fn tag_name(&self) -> String {
        String::new()
    }
}

impl std::ops::Deref for Element {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        static DUMMY_NODE: Node = Node;
        &DUMMY_NODE
    }
}

impl AsRef<Node> for Element {
    fn as_ref(&self) -> &Node {
        static DUMMY_NODE: Node = Node;
        &DUMMY_NODE
    }
}

impl AsRef<Node> for Text {
    fn as_ref(&self) -> &Node {
        static DUMMY_NODE: Node = Node;
        &DUMMY_NODE
    }
}

impl AsRef<Node> for Comment {
    fn as_ref(&self) -> &Node {
        static DUMMY_NODE: Node = Node;
        &DUMMY_NODE
    }
}

pub fn queue_microtask(task: impl FnOnce() + 'static) {
    task();
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

impl<T> CastFrom<Event> for T {
    fn cast_from(source: Event) -> Option<Self> {
        None
    }
}

impl<T> CastFrom<Element> for T {
    fn cast_from(source: Element) -> Option<Self> {
        None
    }
}

pub fn event_target_value<T>(event: &T) -> String {
    String::new()
}

pub fn event_target_checked(ev: &crate::web_sys::Event) -> bool {
    false
}

