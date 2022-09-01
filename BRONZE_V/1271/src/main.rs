macro_rules! u{($e:expr) => ($e.unwrap());}
const SPLIT_SIZE: usize = (u64::MAX / 10) as usize;

struct C {
  v: Vec<u64> // NOTE : 쪼개진 숫자들을 저장하는 벡터 (뒤로 쌓임)
}
impl C {
  fn new() -> Self { Self { v: vec!() }}
  fn push(&mut self, n: u64) { self.v.push(n); }
  fn divide(&mut self, other: &Self) -> Self {
    let mut result = Self::new();
    let mut carry = 0;
    for i in 0..self.v.len() {
      let mut n = self.v[i] + carry;
      if i < other.v.len() { n -= other.v[i]; }
      if n < 10 {
        result.push(n);
        carry = 0;
      } else {
        result.push(n - 10);
        carry = 1;
      }
    }

  }
}

fn main(){
  let mut b=String::new();
  u!(std::io::stdin().read_line(&mut b));
  let v: Vec<u64>=b.split_whitespace().map(|x|x.)).collect();
  print!("{}\n{}\n",v[0]/v[1],v[0]%v[1]);
}


// NOTE : 이어서 쪼개진 숫자들을 다시 합치는 함수 등등 해야됨
