use std::io::{stdin,Read, Stderr};
macro_rules! c { ($e:expr) => {($e - b'0')as i32};}

fn main() -> Result<(), Stderr> {
  let mut v = [0; 3];
  stdin().read_exact(&mut v).unwrap();
  println!("{}", c!(v[0]) - c!(v[1]));
  Ok(())
}
