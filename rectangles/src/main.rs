#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // just an associated fn not a method
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect_1 = Rectangle {
        width: 32,
        height: 16,
    };

    dbg!(&rect_1);

    let rect_area = calculate_area(&rect_1);

    println!("The area is: {}", rect_area);

    let rect_2 = Rectangle {
        width: 60,
        height: 45,
    };

    dbg!(&rect_2);

    println!("Rect 1 can hold rect 2 ?: {}", rect_1.can_hold(&rect_2));

    let square_1 = Rectangle::square(20);
    println!("The new square is {:#?}", square_1);

    dbg!(Rectangle::area(&rect_1));
}

fn calculate_area(rect: &Rectangle) -> u32 {
    println!("The rectangle is {:#?}", rect);
    rect.area()
}
