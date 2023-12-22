mod mqtt_action;

pub struct ActionResult{

}

pub trait ActionArgument{
    fn run(self);

}

pub enum Action{
    Mqtt(Box<dyn ActionArgument>),
}