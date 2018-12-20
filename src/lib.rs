pub trait NodeBase {
    fn to_string(&self) -> String;
}

pub trait AttributeBase {
    fn to_string(&self) -> String;
}

pub trait AttributeImpl {
    const ATTRIBUTE_NAME: &'static str;

    fn value(&self) -> String;
}

impl<T: ?Sized> AttributeBase for T
where
    T: AttributeImpl,
{
    fn to_string(&self) -> String {
        format!(r#"{}="{}""#, Self::ATTRIBUTE_NAME, self.value())
    }
}

pub trait TagBase: NodeBase {
    const TAG_NAME: &'static str;

    type Child: ?Sized;
    type Attribute: ?Sized;

    fn attributes(&self) -> &Vec<Box<Self::Attribute>>;
    fn children(&self) -> &Vec<Box<Self::Child>>;
}

impl<T: ?Sized> NodeBase for T
where
    T: TagBase,
    T::Child: NodeBase,
    T::Attribute: AttributeBase,
{
    fn to_string(&self) -> String {
        let children = self.children();
        let attributes = self
            .attributes()
            .iter()
            .map(|ref attribute| format!(" {}", attribute.to_string()))
            .collect::<String>();

        if children.len() == 0 {
            format!("<{}{} />", Self::TAG_NAME, attributes)
        } else {
            format!(
                "<{0}{2}>{1}</{0}>",
                Self::TAG_NAME,
                children
                    .iter()
                    .map(|ref child| child.to_string())
                    .collect::<String>(),
                attributes
            )
        }
    }
}

macro_rules! make_tag {
    ($name:ident, $children:ident, $attributes:ident) => {
        pub mod $name {
            pub struct Element {
                pub children: Vec<Box<super::$children>>,
                pub attributes: Vec<Box<super::$attributes>>,
            }

            impl crate::TagBase for Element {
                const TAG_NAME: &'static str = stringify!($name);

                type Child = super::$children;
                type Attribute = super::$attributes;

                fn attributes(&self) -> &Vec<Box<super::$attributes>> {
                    &self.attributes
                }

                fn children(&self) -> &Vec<Box<super::$children>> {
                    &self.children
                }
            }
        }
    };
}

pub mod tags {
    pub trait Empty: crate::NodeBase {}
    pub trait FlowElement: crate::NodeBase {}

    use super::attributes::DefaultAttribute;

    make_tag!(span, FlowElement, DefaultAttribute);
    make_tag!(div, FlowElement, DefaultAttribute);
    make_tag!(html, FlowElement, DefaultAttribute);

    impl FlowElement for span::Element {}
    impl FlowElement for div::Element {}

    pub struct Text(pub String);

    impl crate::NodeBase for Text {
        fn to_string(&self) -> String {
            self.0.clone()
        }
    }

    impl FlowElement for Text {}
}

pub mod attributes {
    pub trait None: crate::AttributeBase {}
    pub trait DefaultAttribute: crate::AttributeBase {}

    pub mod class {
        pub struct Attribute(pub String);

        impl crate::AttributeImpl for Attribute {
            const ATTRIBUTE_NAME: &'static str = "class";

            fn value(&self) -> String {
                self.0.clone()
            }
        }
    }

    pub mod id {
        pub struct Attribute(pub String);

        impl crate::AttributeImpl for Attribute {
            const ATTRIBUTE_NAME: &'static str = "id";

            fn value(&self) -> String {
                self.0.clone()
            }
        }
    }

    impl DefaultAttribute for class::Attribute {}
    impl DefaultAttribute for id::Attribute {}
}

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
