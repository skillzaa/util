mod attributesenum;
mod animateresponses;
mod counter;

pub use attributesenum::AttributesEnum;
pub use animateresponses::AnimateResponses;
pub use counter::BaseCounter;
pub trait Animatable {
    fn animate(&self,time_ms:u128)->Option<AnimateResponses>;
}


