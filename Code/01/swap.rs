
// fn swap(x : &mut String, y : &mut String) {
//   let z : String = *x;
//   *x = *y;
//   *y = z;
// }

fn swap(x : &mut String, y : &mut String) {
  let z : String = x.clone();
  *x = y.clone();
  *y = z;
}

use std::mem;

fn main() {
  let mut s1 : String = String::from("hello");
  let mut s2 : String = String::from("world");
  // mem::swap(&mut s1, &mut s2);
  swap(&mut s1, &mut s2);
  println!("{}", s1);
}
