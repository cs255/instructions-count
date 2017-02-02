#![feature(plugin)]
#![plugin(simple_pass)]

static mut X: u32 = 0;

fn main() {
  let mut x = 0;
  let times = 12;
  for _ in 0..times {
    x = x + 1
  }
  println!("{}", x);
  unsafe {
    println!("{}", X);
  }
}
