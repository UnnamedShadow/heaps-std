mod raw_num;

pub mod num;
pub mod str;

use std::fmt::Display;

pub trait printTrait {
    type Output;
    fn print(&self) -> Self::Output;
}
impl<T> printTrait for T
where
    T: Display,
{
    type Output = ();
    fn print(&self) -> Self::Output {
        println!("{}", self);
    }
}

use std::ops::{Add, Div, Mul, Sub};

pub trait addTrait<T> {
    type Output;
    fn add(self, rhs: T) -> Self::Output;
}

impl<T, R> addTrait<R> for T
where
    T: Add<R>,
{
    type Output = <T as Add<R>>::Output;
    fn add(self, rhs: R) -> Self::Output {
        self + rhs
    }
}

pub trait subTrait<T> {
    type Output;
    fn sub(self, rhs: T) -> Self::Output;
}
impl<T, R> subTrait<R> for T
where
    T: Sub<R>,
{
    type Output = <T as Sub<R>>::Output;
    fn sub(self, rhs: R) -> Self::Output {
        self - rhs
    }
}

pub trait mulTrait<T> {
    type Output;
    fn mul(self, rhs: T) -> Self::Output;
}
impl<T, R> mulTrait<R> for T
where
    T: Mul<R>,
{
    type Output = <T as Mul<R>>::Output;
    fn mul(self, rhs: R) -> Self::Output {
        self * rhs
    }
}

pub trait divTrait<T> {
    type Output;
    fn div(self, rhs: T) -> Self::Output;
}
impl<T, R> divTrait<R> for T
where
    T: Div<R>,
{
    type Output = <T as Div<R>>::Output;
    fn div(self, rhs: R) -> Self::Output {
        self / rhs
    }
}
