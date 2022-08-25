use proconio::input;

fn main() {
    input! {
        n:usize, k:usize,
        a:[isize; n], b:[isize; n]
    }
    let diff = (0..n).map(|i| (a[i]-b[i]).abs()).fold(0, |x,y| x+y) as usize;
    println!("{}", if diff<=k && diff%2==k%2 {"Yes"} else {"No"})
}