include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
    println!("The value of the variable is {}", unsafe { SomeVariable });
}
