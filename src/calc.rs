use std::ops::{Div, Mul};

pub fn percent_to_value(total:f64,value:f64)->Option<f64>{
    if total == 0_f64 || value == 0_f64 {
        return None
    }
    let a = value.div(total);
    let b = a.mul(100_f64);
    Some(b)
}
pub fn percentage(total:f64,value:f64)->Option<u128>{
    if total == 0_f64 || value == 0_f64 {
        return None
    }
    let a = total.div(100_f64);
    let b = a.mul(value);
    Some(b.abs() as u128)
}
#[cfg(test)]
#[test]
fn normal(){
    let a:Option<f64> = percent_to_value(1200_f64,600_f64);
    match a {
        Some(x)=>assert!( x == 50_f64),
        None=>(),
    }
}
#[cfg(test)]
#[test]
fn div_by_zero(){
    let a:Option<f64> = percent_to_value(0_f64,600_f64);
    assert!(a==None);
}
#[cfg(test)]
#[test]
fn nominator_zero(){
    let a:Option<f64> = percent_to_value(60_f64,0_f64);
    assert!(a==None);
}
#[cfg(test)]
#[test]
fn both_zero(){
    let a:Option<f64> = percent_to_value(0_f64,0_f64);
    assert!(a==None);
}
#[cfg(test)]
#[test]
fn the_loop(){
    for i in 1..100 {
        let a:Option<f64> = percent_to_value(100_f64,i as f64);
        let ans = i as f64 / 100_f64;
        let ans2 = ans * 100_f64;
        match a {
            Some(x)=>assert!(x == ans2),
            None=>(),
        }
        
    }
}
//================================================
#[cfg(test)]
#[test]
fn div_by_zero_perc(){
    let a:Option<u128> = percentage(0_f64,600_f64);
    assert!(a==None);
}
#[cfg(test)]
#[test]
fn nominator_zero_perc(){
    let a:Option<u128> = percentage(60_f64,0_f64);
    assert!(a==None);
}
#[cfg(test)]
#[test]
fn both_zero_perc(){
    let a:Option<u128> = percentage(0_f64,0_f64);
    assert!(a==None);
}
#[cfg(test)]
#[test]
fn the_loop_perc(){
    for i in 1..100 {
        let a:Option<u128> = percentage(200_f64,i as f64);
        let ans = 200_f64 / 100_f64;
        let ans2 = ans * i as f64;
        match a {
            Some(x)=>assert!(x == ans2 as u128),
            None=>(),
        }
        
    }
}