// use crate::raw_num::RawNum;
// use std::marker::ConstParamTy;

// #[derive(ConstParamTy, PartialEq, Eq)]
// enum SimpleDimension {
//     None,
//     Length,
//     Time,
//     Energy,
// }

// #[derive(ConstParamTy, PartialEq, Eq)]
// struct Dimension<const C: usize> {
//     div_chain: [SimpleDimension; C],
// }

// struct Unit<const C: usize, const U: usize> {
//     dimension: Dimension<C>,
//     symbol: String,
//     no_of_plancks: RawNum<U>,
// }

// pub struct Number<const C: usize, const D: [SimpleDimension; C]> {}
