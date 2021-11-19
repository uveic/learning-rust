use smart_pointer::List::{Cons, Nil};
use smart_pointer::{CustomSmartPointer, MyBox};

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // we create a reference to an i32 value and then use the dereference operator
    // to follow the reference to the data
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // We can rewrite the code to use a Box<T> instead of a reference
    // The only difference is that here we set y to be an instance of a box pointing to a
    // copied value of x rather than a reference pointing to the value of x
    let x1 = 5;
    let y1 = Box::new(x);

    assert_eq!(5, x1);
    assert_eq!(5, *y1);

    let x2 = 5;
    let y2 = MyBox::new(x);

    assert_eq!(5, x2);
    assert_eq!(5, *y2);

    // Deref coercion makes it possible to call hello with a reference to a value of
    // type MyBox<String>
    //
    // Here we’re calling the hello function with the argument &m, which is a reference to a
    // MyBox<String> value. Because we implemented the Deref trait on MyBox<T>, Rust can
    // turn &MyBox<String> into &String by calling deref. The standard library provides an
    // implementation of Deref on String that returns a string slice, and this is in the API
    // documentation for Deref. Rust calls deref again to turn the &String into &str, which
    // matches the hello function’s definition.
    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    // If Rust didn’t implement deref coercion, we would have to write the code in Listing 15-13
    // instead of the code in Listing 15-12 to call hello with a value of type &MyBox<String>.
    let m = MyBox::new(String::from("Pepe"));
    hello(&(*m)[..]);

    let _c = CustomSmartPointer::new(String::from("my stuff"));
    let _d = CustomSmartPointer::new(String::from("other stuff"));
    println!("CustomSmartPointers created.");

    // Rust doesn’t let us call drop explicitly because Rust would still automatically call
    // drop on the value at the end of main. This would be a double free error because Rust
    // would be trying to clean up the same value twice.
    // c.drop();

    let e = CustomSmartPointer::new(String::from("more stuff"));
    println!("CustomSmartPointer created");
    drop(e); // std::mem::drop
    println!("CustomSmartPointer dropped before the end of main.");
}
