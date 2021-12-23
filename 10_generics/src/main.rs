use aggregator::notify;
use aggregator::Tweet;
use aggregator::Summary;
use std::fmt::Display;

// The semicolon tells Rust to load the module from a file with the same name
mod aggregator;

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// we want to compare two values of type T using the greater than (>) operator. Because that
// operator is defined as a default method on the standard library trait std::cmp::PartialOrd,
// we need to specify PartialOrd in the trait bounds for T so the largest function can work on
// slices of any type that we can compare.
// We don’t need to bring PartialOrd into scope because it’s in the prelude.

// With our non-generic versions of the largest function, we were only trying to find the largest
// i32 or char. Types like i32 and char that have a known size can be stored on the stack, so they
// implement the Copy trait. But when we made the largest function generic, it became possible
// for the list parameter to have types in it that don’t implement the Copy trait. Consequently,
// we wouldn't be able to move the value out of list[0] and into the largest variable.
// To call this code with only those types that implement the Copy trait,
// we can add Copy to the trait bounds of T!
fn largest_generic<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

//  If we change the return type to &T instead of T, thereby changing the body of the function
// to return a reference, we wouldn’t need the Clone or Copy trait bounds and we could
// avoid heap allocations.
fn largest_generic_another<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > &*largest {
            largest = &item;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}

// We have to declare T just after impl so we can use it to specify that we’re implementing
// methods on the type Point<T>. By declaring T as a generic type after impl, Rust can identify
// that the type in the angle brackets in Point is a generic type rather than a concrete type.
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn new(x: T, y:T) -> Self {
        Self { x, y }
    }
}

// only implements the cmp_display method if its inner type T implements the PartialOrd trait
// that enables comparison and the Display trait that enables printing
impl<T: Display + PartialOrd> Point<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// We can implement methods only on Point<f32> instances rather than on Point<T> instances
// with any generic type. We use the concrete type f32, meaning we don’t declare any
// types after impl.
impl Point<f32> {
    // Instances of Point<T> where T is not of type f32 will not have this method defined.
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct AnotherPoint<T, U> {
    x: T,
    y: U,
}

impl<T, U> AnotherPoint<T, U> {
    // some generic parameters are declared with impl
    // and some are declared with the method definition
    fn mix_up<V, W>(self, other: AnotherPoint<V, W>) -> AnotherPoint<T, W> {
        AnotherPoint {
            x: self.x,
            y: other.y,
        }
    }
}

// We can also use the impl Trait syntax in the return position to return a value of
// some type that implements a trait
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("pepe"),
        content: String::from("of course I am pepe"),
        reply: false,
        retweet: false,
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![58, 69, 5, 569, 45, 258, 874, 456, 587, 123, 587];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let result = largest_generic(&number_list);
    println!("The largest number is {}", result);

    let res = largest_generic_another(&number_list);
    println!("The largest number is {}", *res);

    let char_list = vec!['a', 'y', 'm', 'q'];
    let result = largest_generic(&char_list);
    println!("The largest char is {}", result);

    let res = largest_generic_another(&char_list);
    println!("The largest char is {}", *res);

    let _integer = Point { x: 5, y: 10};
    let _float = Point { x: 1.0, y: 4.0};
    let _both_integer = AnotherPoint { x: 4, y: 6};
    let _both_float = AnotherPoint { x: 4.0, y: 3.4 };
    let _integer_and_float = AnotherPoint { x: 5, y: 1.23 };

    let p = Point { x: 10, y: 5 };
    println!("p.x = {}", p.x());
    p.cmp_display();

    let p1 = AnotherPoint { x: 5, y: 10.4 };
    let p2 = AnotherPoint { x: "Hello", y: 'c' };
    let p3 = p1.mix_up(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    use aggregator::NewsArticle;

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanly Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    use aggregator::Test;

    let test = Test {
        value: String::from("hello"),
        key: 3,
    };

    println!("Test: {}", test.summarize());
    notify(&tweet);
    notify(&article);
    notify(&returns_summarizable());
}
