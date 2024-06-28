mod names;
mod biology;
mod name_gen;
use biology::*;
pub struct ContextInfo{
    pub most_common:Species, 
    pub second_most_common:Species, 
}
impl ContextInfo{
    pub fn default()->ContextInfo{
        return ContextInfo{most_common:Species::Human, second_most_common:Species::Orc};
    }
}
#[allow(unused)]
pub struct Npc{
    pub first_name:String,
    pub last_name:String,
    pub species:Species,
    pub gender:Gender,
    pub physical_description:Vec<String>, 
    pub personality:Vec<String>,
}
impl Npc{
    pub fn new(info:&ContextInfo)->Npc{
        todo!();
    }
}