use crate::math::Vector2;
#[allow(unused)]
struct Road{
    pub points:Vec<Vector2>
}

impl Road{
    #[allow(unused)]
    pub fn new(points:&[Vector2])->Self{
        let mut v = vec![];
        for p in points{
            v.push(*p);
        }
        return Road{points:v};
    }
    #[allow(unused)]
    pub fn distance_to(&self, point:Vector2){
        
    }
}