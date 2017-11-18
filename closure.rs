fn main() {
  fn function (i: i32) -> i32 { i + 1}
//call function and closures
//println!
  let closure_annoted = |i: i32| -> i32 {i +1};

  let closure_infered = |i     |  i + 1;

  let i = 1;

  let one = || 1;

  println!("closure returing one:{}", one());
}

//Interface and Implementation

//call the function and closures
//
//Rust is great Hindley Miler Type System
//
//trate can for understanding
//Consider all transformations which are pure keep listing the impure one's to one side
//
//Monad
//
//Eva