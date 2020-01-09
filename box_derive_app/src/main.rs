use std::collections::BinaryHeap;
use std::cmp::{Eq,PartialEq,Ordering,Ord};
use std::any::Any;

#[macro_use]
extern crate derive_builder;

#[macro_use]
extern crate command_macro_derive;
use command_macro::CommandMacro; 

/// Application related commands.
pub trait AppCommand : Any + CommandMacro {
    fn execute(&self);
}

impl Eq for Box<dyn AppCommand> {
}

impl PartialOrd for Box<dyn AppCommand> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.command_time().cmp(&self.command_time()))
    }
}

impl Ord for Box<dyn AppCommand> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.command_time().cmp(&self.command_time())
    }
}

impl PartialEq for Box<dyn AppCommand> {
    fn eq(&self, other: &Box<dyn AppCommand>) -> bool {
        self.box_eq(other.as_any())
    }
}

#[derive(Eq, Ord, PartialOrd, PartialEq, Default,
         Builder, Debug, CommandMacro)]
pub struct HelloCommand {
    command_time: u128
}

impl AppCommand for HelloCommand {
    fn execute(&self) {
    }
}

#[derive(Eq, Ord, PartialOrd, PartialEq, Default,
         Builder, Debug, CommandMacro)]
pub struct WorldCommand {
    command_time: u128
}

impl AppCommand for WorldCommand {
    fn execute(&self) {
    }
}


fn main() {
    println!("Entry point");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn equal_test() {
        // Create a command for time 70
        let cmd1:Box<dyn AppCommand> = Box::new(WorldCommandBuilder::default()
                            .command_time(70)
                            .build()
                            .unwrap());

        let cmd2:Box<dyn AppCommand> = Box::new(HelloCommandBuilder::default()
                            .command_time(90)
                            .build()
                            .unwrap());

        println!("equal: {}", &cmd1==&cmd2);
    }
    
    #[test]
    pub fn command_queue_order_test() {
        let mut command_queue: BinaryHeap<Box<dyn AppCommand>> =
            BinaryHeap::new();

        // Create a command for time 70
        let cmd1 = Box::new(WorldCommandBuilder::default()
                            .command_time(70)
                            .build()
                            .unwrap());
        command_queue.push(cmd1); 

        let cmd2 = Box::new(HelloCommandBuilder::default()
                            .command_time(90)
                            .build()
                            .unwrap());
        command_queue.push(cmd2);

        let cmd3 = Box::new(WorldCommandBuilder::default()
                            .command_time(00)
                            .build()
                            .unwrap());
        command_queue.push(cmd3);

        let cmd4 = Box::new(HelloCommandBuilder::default()
                            .command_time(60)
                            .build()
                            .unwrap());
        command_queue.push(cmd4);

	assert_eq!(0, command_queue.pop().unwrap().command_time());
	assert_eq!(60, command_queue.pop().unwrap().command_time());
	assert_eq!(70, command_queue.pop().unwrap().command_time());
	assert_eq!(false, command_queue.is_empty());
	assert_eq!(90, command_queue.pop().unwrap().command_time());
	assert_eq!(true, command_queue.is_empty());
    }
}


