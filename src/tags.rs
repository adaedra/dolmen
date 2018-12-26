pub trait Base {
    fn to_string(&self) -> String;
}

pub trait Tag: Base {
    const TAG_NAME: &'static str;

    type Child: ?Sized;
    type Attribute: ?Sized;

    fn attributes(&self) -> &Vec<Box<Self::Attribute>>;
    fn children(&self) -> &Vec<Box<Self::Child>>;
}

macro_rules! make_tag {
    ($name:ident, $children:ident, $attributes:ident) => {
        pub mod $name {
            pub struct Element {
                pub children: Vec<Box<super::$children>>,
                pub attributes: Vec<Box<super::$attributes>>,
            }

            impl super::Tag for Element {
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

impl<T: ?Sized> Base for T
where
    T: Tag,
    T::Child: Base,
    T::Attribute: crate::attributes::Base,
{
    fn to_string(&self) -> String {
        use crate::attributes::Base;

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

pub trait Empty: Base {}
pub trait FlowElement: Base {}

use super::attributes::DefaultAttribute;

make_tag!(span, FlowElement, DefaultAttribute);
make_tag!(div, FlowElement, DefaultAttribute);
make_tag!(html, FlowElement, DefaultAttribute);

impl FlowElement for span::Element {}
impl FlowElement for div::Element {}

pub struct Text(pub String);

impl Base for Text {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl FlowElement for Text {}
