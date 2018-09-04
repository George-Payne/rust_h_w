#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.height * self.width
    }
    fn can_hold(&self, kinder: &Rect) -> bool {
        self.area() > kinder.area()
    }

    fn square(size: u32) -> Rect {
        Rect {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rect {
        width: 30,
        height: 50,
    };
    let rect2 = Rect::square(10);
    let rect3 = Rect {
        width: 60,
        height: 45,
    };

    println!("the rect is {:?}", rect1);
    println!("The area of the rect is {} pix2", rect1.area());

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
