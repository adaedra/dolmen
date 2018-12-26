pub mod attributes;
pub mod tags;

/// Creates the given node. This macro tries to make a sensible way of writing HTML in rust, without resorting to compiler plugins.
/// Compiler plugins and custom syntaxes are nightly-only while this works in stable.
///
/// The most basic syntax for a tag is:
/// ```
/// # use dolmen::{node, tags::{self, Base}};
/// # assert_eq!(
/// node!(div) // => <div />
/// # .to_string(), "<div />");
/// ````
///
/// You can replace `div` by any other HTML node.
///
/// When you want to add children to a node, you put them into curled braces, separated by commas like in a list:attributes
/// ```
/// # use dolmen::{node, tags::{self, Base}};
/// # assert_eq!(
/// node!(div { node!(span), node!(span) }) // => <div><span /><span /></div>
/// # .to_string(), "<div><span /><span /></div>");
/// ```
///  
/// For inserting text, you can use the `text!` macro:
/// ```
/// # use dolmen::{node, text, tags::{self, Base}};
/// # assert_eq!(
/// node!(div { text!("Hello!") }) // => <div>Hello!</div>
/// # .to_string(), "<div>Hello!</div>");
/// ```
///
/// The macro expects expressions, so you can use conditions and other control structures in your nodes:
/// ```
/// # use dolmen::{node, tags::{self, Base}};
/// # assert_eq!(
/// node!(div {
///     if true {
///         node!(span)
///     } else {
///         node!(div)
///     }
/// })
/// # .to_string(), "<div><span /></div>");
/// ```
///
/// To add attributes, use the following syntax:
/// ```
/// # use dolmen::{node, tags::{self, Base}, attributes};
/// # assert_eq!(
/// node!(div(class: "demo", id: "bar")) // => <div class="demo" id="bar" />
/// # .to_string(), r#"<div class="demo" id="bar" />"#);
/// ```
///
/// Again, the macro awaits expressions on the right side.
///
/// You also have the special `data!` macro to set `data-*` attributes:
/// ```
/// # use dolmen::{node, tags::{self, Base}, attributes, data};
/// # assert_eq!(
/// node!(div(data: data!(foo: "bar"))) // => <div data-foo="bar" />
/// # .to_string(), r#"<div data-foo="bar" />"#);
#[macro_export]
macro_rules! node {
    ($tag:ident) => {
        Box::new(tags::$tag::Element { children: Vec::default(), attributes: Vec::default() })
    };
    ($tag:ident { $( $child:expr ),+ }) => {
        Box::new(tags::$tag::Element { children: vec![ $( $child ),* ], attributes: Vec::default() })
    };
    ($tag:ident ( $( $name:ident : $value:expr ),+ )) => {
        Box::new(tags::$tag::Element { children: Vec::default(), attributes: vec![$( Box::new(attributes::$name::Attribute($value.into())) ),*] })
    };
    ($tag:ident ( $( $name:ident : $value:expr ),+ ) { $( $child:expr ),+ }) => {
        Box::new(tags::$tag::Element { children: vec![ $( $child ),* ], attributes: vec![$( Box::new(attributes::$name::Attribute($value.into())) ),*] })
    };
}

/// Create a simple text node.
#[macro_export]
macro_rules! text {
    ($text:expr) => {
        Box::new(tags::Text($text.into()))
    };
}

/// Creates a HashMap for the `data` attribute. See `html!` for an example.
#[macro_export]
macro_rules! data {
    ( $( $name:ident : $value:expr ),* ) => {{
        use std::collections::HashMap;

        let mut map = HashMap::new();
        $( map.insert(stringify!($name).into(), $value.into()); )*

        map
    }};
}

#[cfg(test)]
mod tests {
    use crate::{
        attributes,
        tags::{self, Base},
    };

    #[test]
    fn test_simple_tag() {
        assert_eq!(node!(div).to_string(), "<div />");
    }

    #[test]
    fn test_tag_with_text() {
        assert_eq!(
            node!(div { text!("Hello, world!") }).to_string(),
            "<div>Hello, world!</div>"
        );
    }

    #[test]
    fn test_tag_with_id() {
        assert_eq!(node!(div(id: "foo")).to_string(), r#"<div id="foo" />"#);
    }

    #[test]
    fn test_tag_with_children() {
        assert_eq!(
            node!(div { node!(span), node!(span) }).to_string(),
            "<div><span /><span /></div>"
        );
    }

    #[test]
    fn test_data() {
        assert_eq!(
            node!(div(data: data!(foo: "bar"))).to_string(),
            r#"<div data-foo="bar" />"#
        );
    }

    fn component(content: &str) -> Box<tags::div::Element> {
        node!(div(class:"component") { text!(content) })
    }

    #[test]
    fn test_component() {
        assert_eq!(
            node!(span { component("Hello!") }).to_string(),
            r#"<span><div class="component">Hello!</div></span>"#
        );
    }
}
