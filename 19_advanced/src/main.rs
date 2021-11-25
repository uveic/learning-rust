use std::fmt;
use std::ops::Add;

// In Rust, global variables are called static variables.
static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

// A type alias makes this code more manageable by reducing the repetition.
type ThisIsATypeAlias = Box<dyn Fn() + Send + 'static>;

fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // Calling an Unsafe Function or Method

    unsafe {
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];

    // Just because a function contains unsafe code doesn't mean we need to mark the entire
    // function as unsafe. In fact, wrapping unsafe code in a safe function is a common
    // abstraction. As an example, split_at_mut, requires some unsafe code.
    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut[1, 2, 3]);
    assert_eq!(b, &mut[4, 5, 6]);

    // Using extern Functions to Call External Code

    // Functions declared within extern blocks are always unsafe to call from Rust code.
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // Accessing or Modifying a Mutable Static Variable

    println!("{}", HELLO_WORLD);

    add_to_counter(5u32);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

    // Default Generic Type Parameters and Operator Overloading
    assert_eq!(
        Point { x: 1, y: 2 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 5 }
    );

    let millimeters = Millimeters(1250);
    let meters = Meters(2);
    assert_eq!(millimeters + meters, Millimeters(3250));

    // Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name

    let person = Human;
    person.fly();

    // To call the fly methods from either the Pilot trait or the Wizard trait,
    // we need to use more explicit syntax to specify which fly method we mean.
    Pilot::fly(&person);
    Wizard::fly(&person);

    println!("A baby dog is called a {}", Dog::baby_name());
    // To disambiguate and tell Rust that we want to use the implementation of Animal for Dog,
    // we need to use fully qualified syntax.
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    // Using Supertraits to Require One Trait’s Functionality Within Another Trait
    let p1 = Point { x: 1, y: 2 };
    p1.outline_print();

    // Using the Newtype Pattern to Implement External Traits on External Types
    //
    // As an example, let’s say we want to implement Display on Vec<T>, which the orphan rule
    // prevents us from doing directly because the Display trait and the Vec<T> type are defined
    // outside our crate. We can make a Wrapper struct that holds an instance of Vec<T>; then we
    // can implement Display on Wrapper and use the Vec<T> value.
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w: {}", w);

    // Creating Type Synonyms with Type Aliases

    // the alias Kilometers is a synonym for i32; unlike the Millimeters and Meters
    type Kilometers = i32;

    let x: i32 = 5;
    let y:Kilometers = 8;
    println!("x + y = {}", x + y);

    // A type alias makes this code more manageable by reducing the repetition.
    let f: ThisIsATypeAlias = Box::new(|| println!("hi"));
    takes_long_type(f);
    let _r = returns_long_type();

    // Function Pointers
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);
}

unsafe fn dangerous() {
    println!("hello, i am dangerous");
}

// `extern` is a keyword that facilitates the creation and use of a Foreign Function Interface
// (FFI). An FFI is a way for a programming language to define functions and enable a different
// (foreign) programming language to call those functions.
extern "C" {
    fn abs(input: i32) -> i32;
}

fn add_to_counter(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Millimeters(u32);

#[derive(Debug, Copy, Clone, PartialEq)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

// Sometimes, you might need one trait to use another trait’s functionality.
// In this case we want OutlinePrint to implement Display to be able to print the point
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl OutlinePrint for Point {}

// If we want to use the trait fmt::Display in the trait OutlinePrint
// we need to rely on the dependent trait also being implemented
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn takes_long_type(f: ThisIsATypeAlias) {
    f();
}
fn returns_long_type() -> ThisIsATypeAlias { Box::new(|| {}) }

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
