#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum AnimateResponses {
   Tf(bool),
   U128(u128),
   F64(f64),
   Txt(String),
   U128f64(u128,f64),
}

