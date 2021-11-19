use std::ops::Deref;

pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Our MyBox<T> type can’t be dereferenced because we haven’t implemented that ability on our type.
// To enable dereferencing with the * operator, we implement the Deref trait.
// The Deref trait, provided by the standard library, requires us to implement one method named
// deref that borrows self and returns a reference to the inner data.
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct CustomSmartPointer {
    data: String,
}

impl CustomSmartPointer {
    pub fn new(data: String) -> CustomSmartPointer {
        CustomSmartPointer { data }
    }
}

// The body of the drop function is where you would place any logic that you wanted to run
// when an instance of your type goes out of scope. We’re printing some text here to
// demonstrate when Rust will call drop.
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
