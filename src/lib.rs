#![feature(adt_const_params)]
#![feature(associated_type_defaults)]

mod raw_num;

pub mod num;
pub mod str;

use std::fmt::Display;

pub trait printTrait<T>
where
    T: Display,
{
    type Output = ();
    fn print(val: T) -> () {
        println!("{}", val);
    }
}

use std::ops::{Add, Div, Mul, Sub};

pub trait addTrait<A, B>
where
    A: Add<B>,
{
    type Output = <A as Add<B>>::Output;
    fn add(lhs: A, rhs: B) -> <A as Add<B>>::Output {
        lhs + rhs
    }
}

pub trait subTrait<A, B>
where
    A: Sub<B>,
{
    type Output = <A as Sub<B>>::Output;
    fn sub(lhs: A, rhs: B) -> <A as Sub<B>>::Output {
        lhs - rhs
    }
}

pub trait mulTrait<A, B>
where
    A: Mul<B>,
{
    type Output = <A as Mul<B>>::Output;
    fn mul(lhs: A, rhs: B) -> <A as Mul<B>>::Output {
        lhs * rhs
    }
}

pub trait divTrait<A, B>
where
    A: Div<B>,
{
    type Output = <A as Div<B>>::Output;
    fn div(lhs: A, rhs: B) -> <A as Div<B>>::Output {
        lhs / rhs
    }
}
