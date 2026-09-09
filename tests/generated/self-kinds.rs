#![allow(dead_code, non_camel_case_types, non_snake_case)]
pub trait Mixed {
    fn inspect(&self) -> String;
    fn factory(&self) -> Self
    where
        Self: Sized;
    fn optional(&self) -> std::option::Option<Self>
    where
        Self: Sized;
    fn nested(&self) -> std::result::Result<std::vec::Vec<Self>, String>
    where
        Self: Sized;
    fn qualified(&self) -> crate::Wrapper<Self>
    where
        Self: Sized;
}
