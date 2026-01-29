pub struct Polygon{
    pub name: String,
    sides : u32,
    pub visible: bool,
}

impl Polygon{
    // Constructor (new)
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            sides: 3,
            visible: true,
        }
    }
    pub fn shape(&self) -> String{
        if self.sides == 3{
            return "triangle".to_string()
        }else if self.sides == 4{
            return "square".to_string()
        }else if self.sides == 5{
            return "pentagon".to_string();
        }
        else{
            return "polygon".to_string()
        }
    }

    pub fn increment_sides(&mut self){
        self.sides += 1;
    }
    pub fn sides(&self) -> u32{
        self.sides
    }
}