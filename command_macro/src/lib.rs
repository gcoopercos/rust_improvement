use std::any::Any;

pub trait CommandMacro {
    fn box_eq(&self, other: &dyn Any) -> bool;
    fn as_any(&self) -> &dyn Any;
    fn command_time(&self) -> u128;
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
