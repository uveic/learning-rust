pub trait Summary {
    // Default implementations: sometimes it’s useful to have default behavior
    // Then, as we implement the trait on a particular type, we can keep or override
    // each method’s default behavior.
    fn summarize_author(&self) -> String {
        String::from("Unknown")
    }

    // Default implementations can call other methods in the same trait, even if those other
    // methods don’t have a default implementation. In this way, a trait can provide a lot
    // of useful functionality and only require implementors to specify a small part of it.
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// Implementing a trait on a type is similar to implementing regular methods. The difference
// is that after impl, we put the trait name that we want to implement, then use the for
// keyword, and then specify the name of the type we want to implement the trait for.
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub struct Test {
    pub value: String,
    pub key: i32,
}

// Implements the default behaviour
impl Summary for Test {}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// This longer form is equivalent to the example above but is more verbose. We place trait bounds
// with the declaration of the generic type parameter after a colon and inside angle brackets.
pub fn notify_again<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// pub fn notify(item1: &impl Summary, item2: &impl Summary) {
// pub fn notify<T: Summary>(item1: &T, item2: &T) {

// Specifying Multiple Trait Bounds with the + Syntax
// pub fn notify(item: &(impl Summary + Display)) {
// pub fn notify<T: Summary + Display>(item: &T) {



// instead of writing this:
//     fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {

// we can use a where clause, like this:
//     fn some_function<T, U>(t: &T, u: &U) -> i32
//         where T: Display + Clone,
//               U: Clone + Debug
//     {

