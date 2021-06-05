use std::fmt;
use super::AnimateResponses;
use super::AttributesEnum;
/// The 'Animatable' is the main trait that is to be implemented by any object in my library if it wants to be animated.The 'Animatable' trait can be implemented by any Type. It means that the said object will take the value of one of the attributes (given in 'AttributesEnum') and animate it (as it feels fit) and return result.  While calling the 'animate()' we do send it the current time ( time from the start of the animation) but it is totally upto the implementation of the trait user if he wants to make use of this time or not.
pub trait Animatable: fmt::Debug {
    fn animate(&self,time_ms:u128)->Option<AnimateResponses>;
    fn get_attr_to_animate(&self)->AttributesEnum;
}
