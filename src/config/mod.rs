pub const TYPE_SPHERE: usize = 1;
pub const TYPE_CUBE: usize = 2;

pub struct Transform {
    pub PositionY: f64,
    pub PositionX: f64,
    pub Scale: f64,
}
impl Transform {
    pub fn new(posy: f64, posx: f64, scale: f64) -> Transform {
        Transform {
            PositionY: posy,
            PositionX: posx,
            Scale: scale,
        }
    }
}

pub struct Object {
    pub Type: usize,
    pub Transform: Transform,
}
impl Object {
    pub fn new(objtype: usize, transform: Transform) -> Object {
        Object {
            Type: objtype,
            Transform: transform,
        }
    }
    pub fn info(&self) {
        let mut restype: String;
        if self.Type == TYPE_SPHERE {restype = "Sphere".to_string()}
        else if self.Type == TYPE_CUBE {restype = "Cube".to_string()}
        else {restype = format!("Unknow format type: {}", self.Type)}
        println!("Object: `{}`; Transform: {}y, {}x, {}s", restype, self.Transform.PositionY, self.Transform.PositionX, self.Transform.Scale);
    }
}

