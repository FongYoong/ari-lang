/*
#[derive(Debug)]
struct Person {
    first_name: &'static str,
    last_name: &'static str
}

impl Person {

    fn new(first: &str, name: &str) -> Person {
        Person {
            first_name: first,
            last_name: name
        }
    }
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
    fn copy(&self) -> Self {
        Self::new(&self.first_name,&self.last_name)
    }
    fn set_first_name(&mut self, name: &str) {
        self.first_name = name.to_string();
    }
    fn to_tuple(self) -> (String,String) {
        (self.first_name, self.last_name)
    }

}
let p = Person::new("John","Smith");
println!("person {} {}", p.first_name,p.last_name);
*/

trait Show {
    fn show(&self) -> String;
}

impl Show for i32 {
    fn show(&self) -> String {
        format!("four-byte signed {}", self)
    }
}

impl Show for f64 {
    fn show(&self) -> String {
        format!("eight-byte float {}", self)
    }
}

fn main() {
    let answer = 42;
    let maybe_pi = 3.14;
    let s1 = answer.show();
    let s2 = maybe_pi.show();
    println!("show {}", s1);
    println!("show {}", s2);
}

/*
use std::fmt;

impl fmt::Debug for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.full_name())
    }
}
...
    println!("{:?}", p);
    // John Smith
*/

/*
fn sqr<T> (x: T) -> T::Output
where T: std::ops::Mul + Copy {
    x * x
}
*/