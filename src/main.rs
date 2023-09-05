mod found_the_secret_number;


struct Rectangle {
    height: i32,
    width: i32,
}

impl Rectangle {
    fn new(height: i32, width: i32) -> Self {
        // Constructor
        // ::new() - because self not called
        Self {
            height,
            width,
        }
    }
    fn area(self: &Self) -> i32 {
        self.height * self.width
    }

    fn perimeter(&self) -> i32 {
        2 * (self.height + self.width)
    }
}

impl Rectangle {
    fn can_catch(&self, another_rectangle: &Rectangle) -> bool {
        self.height >= another_rectangle.height && self.width >= another_rectangle.width ||
            self.height >= another_rectangle.width && self.width >= another_rectangle.height
    }
}

fn main() {
    let rec1 = Rectangle {
        height: 15,
        width: 10,
    };

    let rec2 = Rectangle::new(10, 14);
    println!("{}", rec1.can_catch(&rec2));
}