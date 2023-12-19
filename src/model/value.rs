pub trait IsValue: Clone {}

impl IsValue for bool {}
impl IsValue for String {}

impl IsValue for usize {}
impl IsValue for u8 {}
impl IsValue for u16 {}
impl IsValue for u32 {}
impl IsValue for u64 {}
impl IsValue for u128 {}

impl IsValue for isize {}
impl IsValue for i8 {}
impl IsValue for i16 {}
impl IsValue for i32 {}
impl IsValue for i64 {}
impl IsValue for i128 {}

impl IsValue for f32 {}
impl IsValue for f64 {}
