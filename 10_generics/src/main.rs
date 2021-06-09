fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// fn largest_generic<T>(list: &[T]) -> T {
//     let mut largest = list[0];
//
//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//
//     largest
// }

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct AnotherPoint<T, U> {
    x: T,
    y: U,
}

impl<T, U> AnotherPoint<T, U> {
    fn mix_up<V, W>(self, other: AnotherPoint<V, W>) -> AnotherPoint<T, W> {
        AnotherPoint {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![58, 69, 5, 569, 45, 258, 874, 456, 587, 123, 587];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    // let result = largest_generic(&number_list);
    // println!("The largest number is {}", result);
    //
    // let char_list = vec!['a', 'y', 'm', 'q'];
    // let result = largest_generic(&char_list);
    // println!("The largest char is {}", result);

    let _integer = Point { x: 5, y: 10};
    let _float = Point { x: 1.0, y: 4.0};
    let _both_integer = AnotherPoint { x: 4, y: 6};
    let _both_float = AnotherPoint { x: 4.0, y: 3.4 };
    let _integer_and_float = AnotherPoint { x: 5, y: 1.23 };

    let p = Point { x: 10, y: 5 };
    println!("p.x = {}", p.x());

    let p1 = AnotherPoint { x: 5, y: 10.4 };
    let p2 = AnotherPoint { x: "Hello", y: 'c' };
    let p3 = p1.mix_up(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
