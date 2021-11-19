use std::cell::RefCell;
use smart_pointer_other::RcList::{Cons, Nil};

use smart_pointer_other::AnotherList::Cons as AnotherCons;
use smart_pointer_other::AnotherList::Nil as AnotherNil;

use std::rc::Rc;

fn main() {
    // The Cons variants own the data they hold, so when we create the b list, a is moved
    // into b and b owns a. Then, when we try to use a again when creating c, we’re not
    // allowed to because a has been moved.
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));

    // Instead, we’ll change our definition of List to use Rc<T> in place of Box<T>, as shown
    // in Listing 15-18. Each Cons variant will now hold a value and an Rc<T> pointing to a List.
    // When we create b, instead of taking ownership of a, we’ll clone the Rc<List> that a is
    // holding, thereby increasing the number of references from one to two and letting
    // a and b share ownership of the data in that Rc<List>. We’ll also clone a when creating c,
    // increasing the number of references from two to three. Every time we call Rc::clone,
    // the reference count to the data within the Rc<List> will increase, and the data won’t be
    // cleaned up unless there are zero references to it.
    let d = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("Count after creating d = {}", Rc::strong_count(&d));

    let _e = Cons(3, Rc::clone(&d));
    println!("Count after creating e = {}", Rc::strong_count(&d));

    {
        let _f = Cons(4, Rc::clone(&d));
        println!("Count after creating f = {}", Rc::strong_count(&d));
    }

    println!("Count after f goes out of scope = {}", Rc::strong_count(&d));

    println!("==========================================");

    let g = Rc::new(AnotherCons(5, RefCell::new(Rc::new(AnotherNil))));
    println!("g initial rc count = {}", Rc::strong_count(&g));
    println!("a next time = {:?}", g.tail());

    let h = Rc::new(AnotherCons(10, RefCell::new(Rc::clone(&g))));

    println!("g rc count after h creation = {}", Rc::strong_count(&g));
    println!("h initial rc count = {}", Rc::strong_count(&h));
    println!("h next item = {:?}", h.tail());

    if let Some(link) = g.tail() {
        *link.borrow_mut() = Rc::clone(&h);
    }

    println!("h rc count after changing g = {}", Rc::strong_count(&h));
    println!("g rc count after changing g = {}", Rc::strong_count(&g));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("g next item = {:?}", g.tail());
}
