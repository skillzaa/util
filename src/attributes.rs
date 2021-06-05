#[derive(Debug)]
/// The 'Attributes' struct has all the attributes that any object in my library can have. Using these attributes we can animate the object. For example if we change the 'x' and 'y' of an element its position will change. Each attribute has its own get and set functions eg 'get_color' and 'set_color' for 'color' attributes. when the library becomes more complex all the logic for changing attributes will be placed in these getter and setter functions. 
pub struct Attributes{
    //--strings
    bounding_rectangle_color:String,
    shadow_color:String,
    // name:String, //removed since it is in hasmap that already has a string key so this is confusing
    color:String,
    //Flags
    clockwise:bool,
    filled:bool,
    draw_bounding_rectangle:bool,
    //Numbers
    opacity:u128,
    x:u128,
    y:u128,
    width:u128,
    height:u128,
    start_angle:u128,
    line_width:u128,
    shadow_blur:u128,
    shadow_offset_x:u128,
    shadow_offset_y:u128,
    line_dash_size:u128,
    line_dash_gap:u128,
    bounding_rectangle_padding:u128,
}
impl Attributes {
    pub fn new()->Self{
        Attributes {
        bounding_rectangle_color: String::from("red"),
        shadow_color:String::from("red"),
        // name:String::from("red"), //depricated
        color:String::from("red"),   
        clockwise:true,
        filled:true,
        draw_bounding_rectangle:true,
        opacity:100,
        x:100,
        y:100,
        width:100,
        height:100,
        start_angle:0,
        line_width:2,
        shadow_blur:4,
        shadow_offset_x:1,
        shadow_offset_y:1,
        line_dash_size:10,
        line_dash_gap:0,
        bounding_rectangle_padding:5,
        }
    }
    pub fn set_bounding_rectangle_color(&mut self,v:String){
        self.bounding_rectangle_color = v;
    }
    pub fn get_bounding_rectangle_color(&self)->String{
        String::from(&self.bounding_rectangle_color)
    }
    pub fn set_shadow_color(&mut self,v:String)->String{
        self.shadow_color = v;
        String::from(&self.shadow_color)
        
    }
    pub fn get_shadow_color(&self)->String{
        String::from(&self.shadow_color)
    }

    pub fn set_color(&mut self,v:String)->String{
        self.color = v;
        String::from(&self.color)
        
    }
    pub fn get_color(&self)->String{
        String::from(&self.color)
    }
    pub fn set_clockwise(&mut self,v:bool)->bool{
        self.clockwise = v;
        self.clockwise
        
    }
    pub fn get_clockwise(&self)->bool{
        self.clockwise
    }
    pub fn set_filled(&mut self,v:bool)->bool{
        self.filled = v;
        self.filled
        
    }
    pub fn get_filled(&self)->bool{
        self.filled
    }
    pub fn set_draw_bounding_rectangle(&mut self,v:bool)->bool{
        self.draw_bounding_rectangle = v;
        self.draw_bounding_rectangle
        
    }
    pub fn get_draw_bounding_rectangle(&self)->bool{
        self.draw_bounding_rectangle
    }
    pub fn set_opacity(&mut self,v:u128)->u128{
        self.opacity = v;
        self.opacity   
    }
    pub fn get_opacity(&self)->u128{
        self.opacity
    }
    pub fn set_x (&mut self,v:u128)->u128{
        self.x  = v;
        self.x    
    }
    pub fn get_x (&self)->u128{
        self.x 
    }
    pub fn set_y(&mut self,v:u128)->u128{
        self.y = v;
        self.y   
    }
    pub fn get_y(&self)->u128{
        self.y
    }
    pub fn set_width(&mut self,v:u128)->u128{
        self.width = v;
        self.width   
    }
    pub fn get_width(&self)->u128{
        self.width
    }
    pub fn set_height(&mut self,v:u128)->u128{
        self.height = v;
        self.height   
    }
    pub fn get_height(&self)->u128{
        self.height
    }
    
    pub fn set_start_angle (&mut self,v:u128)->u128{
        self.start_angle  = v;
        self.start_angle    
    }
    pub fn get_start_angle (&self)->u128{
        self.start_angle 
    }
    pub fn set_line_width(&mut self,v:u128)->u128{
        self.line_width = v;
        self.line_width   
    }
    pub fn get_line_width(&self)->u128{
        self.line_width
    }
    pub fn set_shadow_blur(&mut self,v:u128)->u128{
        self.shadow_blur = v;
        self.shadow_blur   
    }
    pub fn get_shadow_blur(&self)->u128{
        self.shadow_blur
    }
    pub fn set_shadow_offset_x(&mut self,v:u128)->u128{
        self.shadow_offset_x = v;
        self.shadow_offset_x   
    }
    pub fn get_shadow_offset_x(&self)->u128{
        self.shadow_offset_x
    }
    pub fn set_shadow_offset_y(&mut self,v:u128)->u128{
        self.shadow_offset_y = v;
        self.shadow_offset_y   
    }
    pub fn get_shadow_offset_y(&self)->u128{
        self.shadow_offset_y
    }
    
    pub fn set_line_dash_size(&mut self,v:u128)->u128{
        self.line_dash_size = v;
        self.line_dash_size   
    }
pub fn get_line_dash_size(&self)->u128{
    self.line_dash_size

}pub fn set_line_dash_gap(&mut self,v:u128)->u128{
    self.line_dash_gap = v;
    self.line_dash_gap   
}
pub fn get_line_dash_gap(&self)->u128{
    self.line_dash_gap

}pub fn set_bounding_rectangle_padding(&mut self,v:u128)->u128{
    self.bounding_rectangle_padding = v;
    self.bounding_rectangle_padding   
}
pub fn get_bounding_rectangle_padding(&self)->u128{
    self.bounding_rectangle_padding
}
   
}//impl Attributes

