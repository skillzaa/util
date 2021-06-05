use std::ops::Mul;

// mod attributesenum;
use super::{AttributesEnum,AnimateResponses,percent_to_value,percentage,Animatable};

#[derive(Copy)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
/// This is one of the Object in my library that implements the 'Animatable' trait. In future there will be more
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
    pub fn new(from_time:f64,to_time:f64,from:u128,to:u128,attr_to_animate:AttributesEnum)->Option<BaseCounter>{
    assert!(from_time < to_time,"From time can not be bigger than To time");   
    assert!(from < 5000,"From value is too large");
    assert!(to < 5000,"To value is too large");
        let from_time_millis = from_time.mul(1000_f64);
        let to_time_millis = to_time.mul(1000_f64);
            Some(BaseCounter {
                from_time : from_time_millis as u128,
                to_time : to_time_millis as u128,
                from,
                to,
                is_reverse: is_reverse(from, to),
                animation_duration : duration(to_time_millis as u128, from_time_millis as u128), 
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

impl Animatable for BaseCounter{

    fn animate(&self, time_ms:u128) ->Option<AnimateResponses> {
        /////////////////////////////
        self.is_time_valid(time_ms)?;
        
        let time_lapsed = self.time_lapsed(time_ms);
        
        let time_perc_lapsed = percent_to_value(self.animation_duration as f64, time_lapsed as f64)?;
          
        let per:Option<f64> = percentage(self.animation_distance as f64,time_perc_lapsed);
            
            match per {
                Some(y)=> {
                    if self.is_reverse == true {
                        let u = self.from  - y as u128;
                        let f = self.from as f64  - y ;
                        return Some(AnimateResponses::U128f64(u,f))
                    }else {
                        let u = y  as u128 + self.from ;
                        let f = y  + self.from as f64;
                        return Some(AnimateResponses::U128f64(u,f))
                    }
                },
                None=> return None,   
            }       
        /////////////////////////////
    }
    fn get_attr_to_animate(&self)->AttributesEnum{
        self.attr_to_animate
    }
    
}//impl Animatable for BaseCounter
