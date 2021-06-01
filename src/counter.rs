
use crate::AttributesEnum;
use crate::AnimateResponses;
use crate::Animatable;

#[derive(Copy)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct BaseCounter {
    from_time:u128,
    to_time:u128,
    from:u128,
    to:u128,
    is_reverse:bool, 
    animation_duration:u128,
    animation_distance:u128,
    attr_to_animate:AttributesEnum,
}
//==========================================

impl BaseCounter{
    pub fn new(from_time:u128,to_time:u128,from:u128,to:u128,attr_to_animate:AttributesEnum)->Option<BaseCounter>{
    assert!(from_time < to_time,"From time can not be bigger than To time");   
    assert!(from < 5000,"From value is too large");
    assert!(to < 5000,"To value is too large");
        let from_time_millis:u128 = from_time * 1000;
        let to_time_millis:u128 = to_time * 1000;
            Some(BaseCounter {
                from_time : from_time_millis,
                to_time : to_time_millis,
                from,
                to,
                is_reverse: is_reverse(from, to),
                animation_duration : duration(to_time_millis, from_time_millis), 
                animation_distance:distance(from, to),
                attr_to_animate:attr_to_animate,
            })
    } 
    pub fn is_time_valid(&self,time_ms:u128)->Option<bool>{
        if time_ms > self.to_time || time_ms < self.from_time {
            return None;
           }else {
               Some(true)
           }       
    }
    pub fn time_lapsed(&self,time_ms:u128)->u128{
        //--time is valid check is seperate.       
        time_ms - self.from_time
    }      
    
    //----we had animate here but now nothing-!!!!!

    pub fn get_attr_to_animate(&self)->AttributesEnum{
        self.attr_to_animate
    } 
    
       
}//end of impl block
//----public API
fn is_reverse(from:u128,to:u128)->bool{
    if from < to {
        return false
    }else {
        return true
    }
}
fn duration(to_time:u128,from_time:u128)->u128{
    if from_time > to_time {
        return from_time - to_time
    }else {
        return to_time - from_time
    }
}
fn distance (from:u128,to:u128)->u128{
    if from > to {
        return from - to
    }else {
        return to - from
    }
}

///////////////////////trait
impl Animatable for BaseCounter{

    fn animate(&self, time_ms:u128) ->Option<AnimateResponses> {
        if time_ms > 0 {
            Some(AnimateResponses::U128(777))
        }else {
            None
        }
    }
    
}//impl Animatable for BaseCounter
