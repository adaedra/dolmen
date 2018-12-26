pub trait Base {
    fn to_string(&self) -> String;
}

pub trait Attribute {
    const ATTRIBUTE_NAME: &'static str;

    fn value(&self) -> String;
}

impl<T: ?Sized> Base for T
where
    T: Attribute,
{
    fn to_string(&self) -> String {
        format!(r#"{}="{}""#, Self::ATTRIBUTE_NAME, self.value())
    }
}

pub trait None: Base {}
pub trait DefaultAttribute: Base {}

pub mod class {
    pub struct Attribute(pub String);

    impl super::Attribute for Attribute {
        const ATTRIBUTE_NAME: &'static str = "class";

        fn value(&self) -> String {
            self.0.clone()
        }
    }
}

pub mod id {
    pub struct Attribute(pub String);

    impl super::Attribute for Attribute {
        const ATTRIBUTE_NAME: &'static str = "id";

        fn value(&self) -> String {
            self.0.clone()
        }
    }
}

impl DefaultAttribute for class::Attribute {}
impl DefaultAttribute for id::Attribute {}

pub mod data {
    use std::collections::HashMap;

    pub struct Attribute(pub HashMap<String, String>);

    impl super::Base for Attribute {
        fn to_string(&self) -> String {
            self.0
                .iter()
                .map(|(ref name, ref value)| format!(r#"data-{}="{}""#, name, value))
                .collect::<Vec<String>>()
                .join(" ")
        }
    }
}

impl DefaultAttribute for data::Attribute {}
