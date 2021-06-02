use std::fmt;
use super::AnimateResponses;
use super::AttributesEnum;
pub trait Animatable: fmt::Debug {
    fn animate(&self,time_ms:u128)->Option<AnimateResponses>;
    fn get_attr_to_animate(&self)->AttributesEnum;
}
