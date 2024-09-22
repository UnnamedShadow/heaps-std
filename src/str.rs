use std::{
    iter::Map,
    ops::{Add, Div, Mul},
    str::Split,
};

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

#[derive(Debug, Clone)]
pub struct Str(String);
impl Add<Str> for Str {
    type Output = Str;
    fn add(self, rhs: Self) -> Self::Output {
        Str(format!("{}{}", self.0, rhs.0))
    }
}
impl Div<Str> for Str {
    type Output = Vec<Str>;
    fn div(self, rhs: Self) -> Self::Output {
        self.0
            .split(rhs.0.as_str())
            .map(|x| Str(x.to_string()))
            .collect()
    }
}
