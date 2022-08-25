use proconio::input;

fn main() {
    /*
        log2(a) < b * log2(c)
    <=> log2(a) < log2(c^b)
    <=> a < c^b
    */
    input! { a: usize, b: u32, c: usize }
    println!("{}", if a < c.pow(b) {"Yes"} else {"No"})
}