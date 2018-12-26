/// Low-level trait for tags. Used for some type magic.
pub trait Base {
    fn to_string(&self) -> String;
}

/// Common format for a tag, implemented tags should implement this trait.
///
/// Implementation of this trait is taken care of by the internal
/// `make_tag!` macro, so you should not have to implement it by yourself
/// when adding new tags to the crate.
pub trait Tag: Base {
    /// The tag name as it will be shown in HTML
    const TAG_NAME: &'static str;

    /// Trait to be implemented by the valid children of this node
    type Child: ?Sized;
    /// Trait to be implemented by the valid attributes of this node
    type Attribute: ?Sized;

    /// Returns the current attributes of the node
    fn attributes(&self) -> &Vec<Box<Self::Attribute>>;
    /// Returns the current children of the node
    fn children(&self) -> &Vec<Box<Self::Child>>;
}

/// Declare a new tag module and implements it as a standard tag.
/// Give it the tag name as an identifier which will be used to name the module,
/// and the restricting traits of the children nodes and attributes for this
/// HTML element.
macro_rules! make_tag {
    ($name:ident, $children:ident, $attributes:ident) => {
        /// The module for the tag, its name is the one you use with the `node!` macro.
        pub mod $name {
            /// The element representation
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

/// Pseudo group indicating an element should never have children.
/// No tags should implement this interface.
pub trait Empty: Base {}

/// HTML flow elements.
pub trait FlowElement: Base {}

use super::attributes::DefaultAttribute;

make_tag!(span, FlowElement, DefaultAttribute);
make_tag!(div, FlowElement, DefaultAttribute);
make_tag!(html, FlowElement, DefaultAttribute);

impl FlowElement for span::Element {}
impl FlowElement for div::Element {}

/// Represents a text node alone. Converted to its contents when transformed into string.
pub struct Text(pub String);

impl Base for Text {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl FlowElement for Text {}
