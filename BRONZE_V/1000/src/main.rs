fn main() {
  let mut buf: String = String::new();
  std::io::stdin().read_line(&mut buf).unwrap();
  let sum: i32 = buf.split_whitespace().map(|x| x.parse::<i32>().unwrap()).sum();
  println!("{}",sum);
}
