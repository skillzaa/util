use bilzaa2dutil::AttributesEnum;
use bilzaa2dutil::BaseCounter as Animation;
use bilzaa2dutil::Animatable;
use bilzaa2dutil::AnimateResponses;

//use #[should_panic] with the test --if to check errors 
fn test_a(a:Animation,time_ms:u128,answer:AnimateResponses){
    match a.animate(time_ms) {
        Some(x)=> assert_eq!(x,answer),
        None=> panic!("Panic"),
    }
}
fn get_a(from_time:f64,to_time:f64,from:u128,to:u128)->Animation{
    let a = Animation::new(from_time, to_time, from, to, AttributesEnum::Width);
    match a {
        Some(b)=>{return b},
        None=>panic!("Animation creation error"),
}
}
//////////////////////////////////
// #[cfg(test)]
#[test]
fn get_attr(){
  let a:Animation = get_a(0.0,10.0,0,100); 
  let ata = a.get_attr_to_animate();
 assert_eq!(ata,AttributesEnum::Width);
}
#[test]
fn first(){
  let a:Animation = get_a(0.0,10.0,0,100);  
 test_a(a, 10000, AnimateResponses::U128f64(100,100.0));
}
#[test]
fn mm(){
  let a:Animation = get_a(0.0,10.0,0,100);  
 test_a(a, 5000, AnimateResponses::U128f64(50,50.0));
}
#[test]
#[should_panic] // time is not right it shd be 15000
fn mmm(){
  let a:Animation = get_a(10.0,20.0,0,100);  
 test_a(a, 15, AnimateResponses::U128f64(50,50.0));
}
#[test]
#[should_panic]
fn auto_second(){
  let a:Animation = get_a(0.0,10.0,0,100);  
 test_a(a, 0, AnimateResponses::U128f64(0,0.0));
}
#[test]
fn aa(){
  let a:Animation = get_a(10.0,20.0,0,100);  
 test_a(a, 15000, AnimateResponses::U128f64(50,50.0));
}
#[test]
fn aaa(){
  let a:Animation = get_a(10.0,20.0,100,200);  
 test_a(a, 15000, AnimateResponses::U128f64(150,150.0));
}
#[test]
fn reverse_one(){
  let a:Animation = get_a(10.0,20.0,200,100);  
 test_a(a, 15000, AnimateResponses::U128f64(150,150.0));
}
#[test]
fn reverse_two(){
  let a:Animation = get_a(10.0,20.0,200,100);  
 test_a(a, 20000, AnimateResponses::U128f64(100,100.0));
}
#[test]
#[should_panic] // time is not vlaid
fn reverse_three(){ // its zero
  let a:Animation = get_a(10.0,20.0,200,100);  
 test_a(a, 0, AnimateResponses::U128f64(200,200.0));
}
#[test]
#[should_panic]
fn reverse_four(){ // its zero--start time--will ret none
  let a:Animation = get_a(10.0,20.0,200,100);  
 test_a(a, 10000, AnimateResponses::U128f64(200,200.0));
}
#[test]
fn reverse_five(){//50%
  let a:Animation = get_a(10.0,50.0,200,100);  
 test_a(a, 30000, AnimateResponses::U128f64(150,150.0));
}
#[test]
fn reverse_six(){//its 175 not 125 its reverse
  let a:Animation = get_a(10.0,50.0,200,100);  
 test_a(a, 20000, AnimateResponses::U128f64(175,175.0));
}
#[test]
fn normal(){//its 125 now since not reverse
  let a:Animation = get_a(10.0,50.0,100,200);  
 test_a(a, 20000, AnimateResponses::U128f64(125,125.0));
}
#[test]
fn normal_two(){//end
  let a:Animation = get_a(10.0,50.0,100,200);  
 test_a(a, 50000, AnimateResponses::U128f64(200,200.0));
}
#[test]
#[should_panic]
fn normal_three(){//end
  let a:Animation = get_a(10.0,50.0,100,200);  
 test_a(a, 10000, AnimateResponses::U128f64(100,100.0));
}
#[test]
#[should_panic]
fn time_wrong(){//end
  let a:Animation = get_a(10.0,5.0,100,200);  
 test_a(a, 10000, AnimateResponses::U128f64(100,100.0));
}