struct CustomSmartPoiner {
    data: String,
}

impl Drop for CustomSmartPoiner {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPoiner {
        data: String::from("my stuff"),
    };

    let d = CustomSmartPoiner {
        data: String::from("Other stuff"),
    };
    println!("CustomSmartPointer created!");
}
