use proconio::input;

fn main() {
  input!{
    n: usize,
    mut a: [usize; n],
    q: usize,
    b: [usize; q]
  }

  a.sort();
  
  for student_rate in b {
    match a.binary_search(&student_rate) {
      Ok(_) => println!("0"),
      Err(j) => println!("{}", match j {
          i if i==n => student_rate - a[n-1],
          0 => a[0] - student_rate,
          _ => (a[j] - student_rate).min(student_rate - a[j-1]),
      })
    }
  }
}
