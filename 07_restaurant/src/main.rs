use std::collections::HashMap;
use std::fmt;

use std::fmt::Result;
use std::io::Result as IoResult;

use std::{cmp::Ordering, io};

use std::collections::*;
use restaurant::eat_at_restaurant;

// fn function1() -> fmt::Result {
//     // do something
// }
//
// fn function2() -> io::Result<()> {
//     // do something
// }

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);

    eat_at_restaurant();
}
