mod attributesenum;
mod animateresponses;
mod counter;
use std::fmt;
// use std::fmt::{Debug,Formatter,Result};
pub use attributesenum::AttributesEnum;
pub use animateresponses::AnimateResponses;
pub use counter::BaseCounter;
pub trait Animatable: fmt::Display {
    fn animate(&self,time_ms:u128)->Option<AnimateResponses>;
}

// impl Debug for Animatable {
//     fn fmt(&sel, f: &mut Formatter<'_>) -> Result {
       
        
//     }
// }


