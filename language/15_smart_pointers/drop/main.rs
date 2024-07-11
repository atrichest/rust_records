struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data '{}'!", self.data);
    }
}

use std::mem::drop;

fn main() {
    {
        //A CustomSmartPointer struct that implements the Drop trait where we would put our cleanup code
        let c = CustomSmartPointer {
            data: String::from("my stuff"),
        };
        let d = CustomSmartPointer {
            data: String::from("other stuff"),
        };
        println!("CustomSmartPointers created.");
    }

    {
        let c = CustomSmartPointer {
            data: String::from("some data"),
        };
        println!("CustomSmartPointer created.");
        //Attempting to call the drop method from the Drop trait manually to clean up early
        //error[E0040]: explicit use of destructor method
        //c.drop();
        println!("CustomSmartPointer dropped before the end of main.");
    }
    {
        let c = CustomSmartPointer {
            data: String::from("some data"),
        };
        println!("CustomSmartPointer created.");
        //Calling std::mem::drop to explicitly drop a value before it goes out of scope
        drop(c);
        println!("CustomSmartPointer dropped before the end of main.");
    }
}
