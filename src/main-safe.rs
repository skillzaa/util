use bilzaa2dutil::{BaseCounter,AnimateResponses,AttributesEnum,Animatable};
fn main(){

let a = BaseCounter::new(0,
    10,1,100,AttributesEnum::Width);
match a {
    Some(aa)=>{
        let jj = aa.animate(20);
        match jj {
            Some(jjj)=>println!("{:?}",jjj),
            None=>(),
        }
        
    },
    None=>(),
}




}//main