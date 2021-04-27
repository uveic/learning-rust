#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool
    {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple((width1, height1))
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle {:?} is {} square pixels.",
        &rect1,
        area_struct(&rect1)
    );

    let rect2 = Rectangle {
        width: 20,
        height: 15,
    };

    println!("rec2 is {:?}", rect2);
    println!("rec2 is {:#?}", rect2);

    println!(
        "The area of the rectangle {:?} is {} square pixels.",
        rect1,
        rect1.area()
    );

    println!("Can rect2 hold rect1? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect2? {}", rect2.can_hold(&rect1));

    let sq = Rectangle::square(12);
    println!("Can rect1 hold sq? {}", rect2.can_hold(&sq));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple (dimesions: (u32, u32)) -> u32 {
    dimesions.0 * dimesions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}
