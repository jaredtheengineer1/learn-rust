fn main() {
    // immuatable();
    mutable();
    shadowed();
}

// fn immuatable() {
//   let x = 5;
//   println!("The immutable value of x is: {x}");
//   x = 6;
//   println!("The immuatable value of x is: {x}");
// }

fn mutable() {
  let mut x = 5;
  println!("The mutable value of x is: {x}");
  x = 6;
  println!("The mutable value of x is: {x}");
}

fn shadowed() {
  let x = 5;
  println!("Unshadowed x is {x}");
  let x = x + 1;
  println!("The shadowed value of x is {x}");
  {
    let x = x * 2;
    println!("The inner most shadowed scope of x is {x} (because it was already shadowed to x + 1)");
  }

  println!("After the inner most scope has ended, the shadowed value of x is: {x}");
}