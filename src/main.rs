mod r#trait;
mod lifetimes;


fn main() {
    println!("Hello, world!");
    display_output(&"Hello world".to_string());

    let x = String::from("Dan");
    let z;
    {
        let y = String::from("Eric");
        z = longest(&x, &y);
        println!("{z}");
    }


    //struct with lifetime
    let i: ImportantExcerpt;
    {
        let x = String::from("Dane");
        i = ImportantExcerpt {
            part: &x
        };
    }

    println!("{}", i.part);




}

//a generic function
fn display_output<T: std::fmt::Display>(input: &T) {
    println!("The input is: {}", input);
}

//a generic struct
struct Point<T> {
    x: T,
    y: T,
}

impl<T: std::cmp::PartialOrd + std::fmt::Display> Point<T> {
    fn greatest_cood(&self) {
        if self.x > self.y {
            println!("The greatest value is x");
        } else {
            println!("The greatest value is y");
        }
    }
}

//enum generic
enum ExampleEnum<T, U> {
    Hello(T),
    Point{
        x: U,
        y: U
    }
}

// impl<T, U> Point<T, U> {
//     fn mixup<V, W>(&self, &Other: Point<V, W>) -> Point<T, W> {
//         Point {
//             x: Self.x,
//             y: Other.y
//         }
//     }
// }

//the smallest lifetime is the one that gets taken altogether
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

//struct with lifetimes
struct ImportantExcerpt<'a> {
    part: &'a str
}