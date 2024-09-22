use std::{
    fmt::Display,
    ops::{Add, Div},
};

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

#[derive(Debug, Clone)]
pub struct Str(pub String);
impl Add<Str> for Str {
    type Output = Str;
    fn add(self, rhs: Self) -> Self::Output {
        Str(format!("{}{}", self.0, rhs.0))
    }
}
impl Display for Str {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
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
