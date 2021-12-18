// Primitive str = Immutable fixed-length string somwhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
  // Primitive str
  let hello_prim = "Hello";

  // String
  let mut hello = String::from("Hello ");

  // Get length
  println!("Length: {}", hello.len());

  // Push char
  hello.push('W');

  // Push string
  hello.push_str("orld!");

  println!("{}", hello);
}
