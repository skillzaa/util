#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]

/// When we animate an attribute of a Type the result (the new created value) can be a String a bool a float or any mixture of these. In rust when ever you want to send back multiple types of data from "A" function you have to create an Enum of all the possible return data types. These are all the possible accepted legit results from 'animate' function.
pub enum AnimateResponses {
   Tf(bool),
   U128(u128),
   F64(f64),
   Txt(String),
   U128f64(u128,f64),
}

