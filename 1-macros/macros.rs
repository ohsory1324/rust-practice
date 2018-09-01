fn main() {
  /*
   *  1. Comments are the same with C 
   *  2. macros: Macros look like functions, except that their name ends with a bang '!' (https://doc.rust-lang.org/rust-by-example/macros.html)
   *   2-1. format! : Creates a String using interpolation of runtime expressions.
   *   2-2. print! : same as format! but the text is printed to the console (io::stdout).
   *   2-3. println!: same as print! but a newline is appended.
   *   2-4. eprint!: same as format! but the text is printed to the standard error (io::stderr).
   *   2-5. eprintln!: sames as eprint! but a newline is appended.
   */
  println!("{}", format!("Hello"));                 // => "Hello"
  println!("{}", format!("Hello, {}!", "world"));   // => "Hello, world!"
  println!("{}", format!("The number is {}", 1));   // => "The number is 1"
  println!("{}", format!("{:?}", (3, 4)));          // => "(3, 4)"
  println!("{}", format!("{value}", value=4));      // => "4"
  println!("{}", format!("{} {}", 1, 2));           // => "1 2"
  println!("{}", format!("{:04}", 42));             // => "0042" with leading zeros
}