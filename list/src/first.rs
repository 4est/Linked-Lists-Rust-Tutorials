//While enums let us declare a type that can contain one of several values, 
//structs let us declare a type that contains many values at once
struct Node {
    elem: i32,
    next: List,
}

pub enum List {
    Empty,
    More(Box<Node>),
}