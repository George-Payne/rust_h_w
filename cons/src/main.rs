use std::rc::Rc;
use List::{Cons, Nil};
use Listington::{Consington, Nilington};

enum List {
    Cons(i32, Box<List>),
    Nil,
}

enum Listington {
    Consington(i32, Rc<Listington>),
    Nilington,
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let list2 = Rc::new(Consington(
        1,
        Rc::new(Consington(2, Rc::new(Consington(3, Rc::new(Nilington))))),
    ));

    println!("count after creating list2 = {}", Rc::strong_count(&list2));
    let _b = Consington(3, Rc::clone(&list2));
    println!("count after creating b = {}", Rc::strong_count(&list2));
    {
        let _c = Consington(4, Rc::clone(&list2));
        println!("count after creating c = {}", Rc::strong_count(&list2));
    }
    println!(
        "count after c goes out of scope = {}",
        Rc::strong_count(&list2)
    );

    let _c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointers created.");
    drop(_c);
    println!("CustomSmartPointer dropped before the end of main.");
}
