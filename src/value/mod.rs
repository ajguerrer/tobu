pub mod number;

use std::collections::HashMap;

use self::number::Number;

#[derive(Debug, Clone)]
pub enum Value {
    Bool(bool),
    Number(Number),
    Enum(Enum),
    Bytes(Bytes),
    String(String),
    Message(Message),
    List(List),
    Map(Map),
}

pub type Bytes = Vec<u8>;
pub type Enum = i32;
pub type Message = Vec<Value>;
pub type List = Vec<Value>;
pub type Map = HashMap<Value, Value>;
