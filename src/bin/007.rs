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
      Ok(_) => println!("{}", 0),
      Err(index) => {
        if index == 0 {
          println!("{}", a[0] - student_rate);
        } else if index == n {
          println!("{}", student_rate - a[n-1]);
        } else {
          println!("{}",
            (a[index] - student_rate).min(student_rate - a[index-1])
          )
        } 
      }
    }
  }
}
