struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn perimeter(&self) -> u32 {
        2 * self.width * self.height
    }

    fn debug() -> u32 {
        return 1;
    }
}

fn main() {
    let rect1 = Rect {
        width: 10,
        height: 20,
    };
    println!("area is {}", rect1.area());
    println!("perimeter  is {}", rect1.perimeter());
    println!("Static function is {}", Rect::debug());
}
