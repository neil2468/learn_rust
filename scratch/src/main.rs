fn main() {
    let a = 20;
    let add_to_a: dyn Fn(i32) -> i32 = |b| a + b;
}
