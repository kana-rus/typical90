use proconio::input;

fn gcd(x:usize, y:usize) -> usize {if y==0 {x} else {gcd(y, x%y)}}

fn main() {
    input! { a:usize, b:usize, c:usize }    
    let gcd = gcd(a, gcd(b, c));

    // 長さ L = l * s の辺を長さ gcd の辺 s 個に分割するには (s - 1) 回分割することになる
    /* ex) 4 = 1 * 4
              |---|---|---|---|
        1 cut |---|  |---|---|---|
        2 cut |---|  |---|  |---|---|
        3 cut |---|  |---|  |---|  |---|
    */

    println!("{}", (a / gcd - 1) + (b / gcd - 1) + (c / gcd - 1))
}