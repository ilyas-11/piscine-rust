#[derive(Debug, Clone, Copy)]
pub struct Circle {
	pub center :Point,
	pub radius :f64,
}

impl Circle {
    pub fn new(x: f64,y:f64,red:f64)->Self{
        Self{
            center : Point(x,y),
            radius : red,
        }
    }
    pub fn diameter(&self)->f64{
        self.radius * 2.0
    }
    pub fn area(&self)->f64{
        std::f64::consts::PI *self.radius*self.radius
    }
    pub fn intersect(&self,other:Circle)->bool{
        self.center.distance(other.center)<=self.radius+other.radius
    }
}

#[derive(Debug, Clone, Copy)]
//Tuple Struct
pub struct Point(pub f64,pub f64);

impl Point {
    pub fn distance(&self,other:Point)->f64{
       ((other.0-self.0).powi(2)+(other.1-self.1).powi(2)).sqrt()
    }
}
