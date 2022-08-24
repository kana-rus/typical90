use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [isize; n],
        mut b: [isize; n]
    }
    a.sort_unstable();
    b.sort_unstable();
    
    let ans = (0..n).fold(0, |sum,i| sum + (a[i]-b[i]).abs());
    println!("{}", ans);
}