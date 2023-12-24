use std::collections::HashMap;

pub enum RunTimeData{
    Number(f64),
    String(String),
    Bool(bool),
    Function(Box<dyn Fn(&RunTimeData) -> RunTimeData>),
}
pub struct RuntimeEnvironment{
    environment: HashMap<String,RunTimeData>
}

impl RuntimeEnvironment{
    pub fn new()->RuntimeEnvironment{
        RuntimeEnvironment{
            environment: HashMap::new()
        }
    }

    pub fn insert(&mut self, key: String,data: RunTimeData){
        self.environment.insert(key,data);
    }

    pub fn get(&self,key:&String)->Option<&RunTimeData>{
        self.environment.get(key)
    }


}

