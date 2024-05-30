/*
THE THREE RULES
1. Each parameter that is a reference gets its onw lifetime parameter
2. If there is exactly one input lifetime parameter that lifetime is assigned to all output lifetime parameters
3. If there are multiple input lifetime parameters, but one of the is &self or &mut self the lifetime of self
is assigned to all output lifetime parameters


*/





/*
a dangling reference
let r: &i32;

    {
        let x: i32 = 5;
        r = &x;
    }

println!("r: {}", r);
*/

//the smallest lifetime is the one that gets taken altogether
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
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

impl <'a> ImportantExcerpt<'a> {
    fn return_part(&self, announcement: &str) -> &str {
        self.part
    }
}

//i don't really need to specify the lifetime here because the function has one parameter
//thus a default lifetime
pub fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]
        }
    }

    &s[..]
}

