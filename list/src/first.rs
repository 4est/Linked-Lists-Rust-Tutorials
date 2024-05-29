//While enums let us declare a type that can contain one of several values, 
//structs let us declare a type that contains many values at once

//The internals of an enum are public, so we are not allowed to talk about private types publicly, so make a list struct.
pub struct List {
    head: Link, //the size of the list is the same as the field since it's a single field.
}
enum Link {
    Empty,
    More(Box<Node>)
}
struct Node {
    elem: i32,
    next: Link,
}

//To associate actual code with a type use impl block.
//Need a way to construct a list.
//Self is an alias for the type next to impl below
//We refer to variants of an enum using ::, which is the namespacing operator.
//the last expression of a function is implicitly returned. You can still use 'return' though.
impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }
}