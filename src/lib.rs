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
        print!("{}", self);
    }
}

use std::ops::{Add, Div, Mul, Sub};

pub trait addTrait<T> {
    type Output;
    fn add(&self, rhs: T) -> Self::Output;
}
impl<T, U, O> addTrait<T> for U
where
    U: Clone,
    U: Add<T, Output = O>,
{
    type Output = O;
    fn add(&self, rhs: T) -> Self::Output {
        self.clone() + rhs
    }
}

pub trait subTrait<T> {
    type Output;
    fn sub(&self, rhs: T) -> Self::Output;
}
impl<T, U, O> subTrait<T> for U
where
    U: Clone,
    U: Sub<T, Output = O>,
{
    type Output = O;
    fn sub(&self, rhs: T) -> Self::Output {
        self.clone() - rhs
    }
}

pub trait mulTrait<T> {
    type Output;
    fn mul(&self, rhs: T) -> Self::Output;
}
impl<T, U, O> mulTrait<T> for U
where
    U: Clone,
    U: Mul<T, Output = O>,
{
    type Output = O;
    fn mul(&self, rhs: T) -> Self::Output {
        self.clone() * rhs
    }
}

pub trait divTrait<T> {
    type Output;
    fn div(&self, rhs: T) -> Self::Output;
}
impl<T, U, O> divTrait<T> for U
where
    U: Clone,
    U: Div<T, Output = O>,
{
    type Output = O;
    fn div(&self, rhs: T) -> Self::Output {
        self.clone() / rhs
    }
}
