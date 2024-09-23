#![feature(adt_const_params)]
#![feature(associated_type_defaults)]

mod raw_num;

pub mod num;
pub mod str;

use std::fmt::Display;

pub trait printTrait
where
    Self: Display,
{
    type Output = ();
    fn print(&self) -> () {
        println!("{}", self);
    }
}

use std::ops::{Add, Div, Mul, Sub};

pub trait addTrait<T>
where
    Self: Clone,
    Self: Add<T>,
{
    type Output = <Self as Add<T>>::Output;
    fn add(self, rhs: T) -> <Self as Add<T>>::Output {
        self + rhs
    }
}

pub trait subTrait<T>
where
    Self: Clone,
    Self: Sub<T>,
{
    type Output = <Self as Sub<T>>::Output;
    fn sub(self, rhs: T) -> <Self as Sub<T>>::Output {
        self - rhs
    }
}

pub trait mulTrait<T>
where
    Self: Clone,
    Self: Mul<T>,
{
    type Output = <Self as Mul<T>>::Output;
    fn mul(self, rhs: T) -> <Self as Mul<T>>::Output {
        self * rhs
    }
}

pub trait divTrait<T>
where
    Self: Clone,
    Self: Div<T>,
{
    type Output = <Self as Div<T>>::Output;
    fn div(self, rhs: T) -> <Self as Div<T>>::Output {
        self / rhs
    }
}
