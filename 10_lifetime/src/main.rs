// Rust can’t tell whether the reference being returned refers to x or y. Actually,
// we don’t know either, because the if block in the body of this function returns a
// reference to x and the else block returns a reference to y!
//
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

use std::fmt::Display;

// The function signature now tells Rust that for some lifetime 'a, the function takes two
// parameters, both of which are string slices that live at least as long as lifetime 'a.
// The function signature also tells Rust that the string slice returned from the function
// will live at least as long as lifetime 'a. In practice, it means that the lifetime of the
// reference returned by the longest function is the same as the smaller of the lifetimes of
// the references passed in.
fn longest_with_lifetime_annotation<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// This annotation means an instance of ImportantExcerpt can’t outlive the reference it holds
// in its part field.
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn longest_with_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    // let result = longest(string1.as_str(), string2);

    let result = longest_with_lifetime_annotation(string1.as_str(), &string2);
    println!("The longest string is {}", result);

    let string3 = String::from("long string is long");

    {
        let string4 = String::from("xyz");
        let result = longest_with_lifetime_annotation(string3.as_str(), string4.as_str());
        println!("The longest string is {}", result);
    }

    // Doesn't compile because string5 would need to be valid until the end of the outer scope
    //
    // let res;
    // {
    //     let string5 = String::from("xyz");
    //     res = longest_with_lifetime_annotation(string3.as_str(), string5.as_str());
    // }
    // println!("The longest string is {}", res);

    // &i32        // a reference without a lifetime parameter
    // &'a i32     // a reference with an explicit lifetime
    // &'a mut i32 // a mutable reference with an explicit lifetime

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("Important sentence: {}", i.part);
    let part = i.announce_and_return_part(String::from("this is an announcement").as_str());
    println!("The announcement was done. Level: {}. Part: {}", i.level(), part);

    // 'static is a special lifetime, which means that this reference can live for the
    // entire duration of the program
    let s: &'static str = "I have a static lifetime";
    println!("{}", s);

    let longest = longest_with_announcement(
        string3.as_str(),
        string1.as_str(),
        String::from("ARE YOU THERE?")
    );
    println!("The longest string is {}", longest);
}
