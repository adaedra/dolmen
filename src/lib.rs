pub mod attributes;
pub mod tags;

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

#[macro_export]
macro_rules! text {
    ($text:expr) => {
        Box::new(tags::Text($text.into()))
    };
}

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
