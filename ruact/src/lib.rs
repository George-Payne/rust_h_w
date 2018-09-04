pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw()
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("---{}---", self.label);
    }
}

pub struct Text {
    pub value: String,
}

impl Draw for Text {
    fn draw(&self) {
        println!("{}", self.value);
    }
}
