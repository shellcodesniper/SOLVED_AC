// macro_rules! c { ($e:expr) => {($e)as f64};}
macro_rules! u{($e:expr) => ($e.unwrap());}
fn main(){
  let mut b=String::new();
  u!(std::io::stdin().read_line(&mut b));
  let v: Vec<f64>=b.split_whitespace().map(|x|u!(x.parse())).collect();
  println!("{}",v[0]/v[1]);
}
// SHORT CODING
