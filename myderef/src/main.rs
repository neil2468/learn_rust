use std::ops::Deref;

fn main() {
    println!("Hello, world!");

    let f = Foo::new(112);
    println!("{f:?}");

    let ff: &[i32] = &f;
    println!("{ff:?}");
}

#[derive(Debug)]
struct Foo {
    a: i32,
    b: i32,
    spare: [i32; 2],
}

impl Foo {
    fn new(val: i32) -> Foo {
        Foo {
            a: val % 10,
            b: val / 10,
            spare: [val % 10, val / 10],
        }
    }
}

// impl Deref for Foo {
//     type Target = [i32];

//     fn deref(&self) -> &Self::Target {
//         &self.spare
//     }
// }
