#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Foo {
    Bar = 0,
    Qux = 1,
}
pub const Neg_MinusOne: Neg = -1;
pub const Neg_One: Neg = 1;
pub type Neg = i32;