//! Currently I am writing a 2d animation library in Rust. This crate
//! is a collection of small but useful utilities that I have created
//! for my library. I am making this available (with reasonable 
//! documentation) to any one who wish to use it.
//! ----------------------------------------------------

//Attributes Enum
mod attributesenum;
pub use attributesenum::AttributesEnum;

//Animatable Trait
mod animatable;
pub use animatable::Animatable;

//Animate Responses
mod animateresponses;
pub use animateresponses::AnimateResponses;

//BaseCounter
mod basecounter;
pub use basecounter::BaseCounter;
// calc
mod calc;
pub use calc::{percent_to_value,percentage};
//attributes
mod attributes;
pub use attributes::Attributes;
