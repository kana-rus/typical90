use proconio::input;

const PI: f64 = std::f64::consts::PI;
fn main() {
    input! {
        t: usize,
        l: usize, x: usize, y: usize,
        q: usize
    }
    
    let query = |e: f64| -> f64 {
        let deg = (2.0 * PI) * e / t as f64;
        let (cx, cy, cz) = (0_f64, - (l as f64 / 2.0 * deg.sin()), (l as f64) / 2.0 *(1.0 - deg.cos()));
        let angle = cz.atan2(((cx - x as f64).powi(2) + (cy - y as f64).powi(2)).sqrt());
        180.0 * angle / PI
    };

    print!("{}", {
        let mut ans = String::new();
        for _ in 0..q {
            input! { e: usize }
            ans += &(format!("{:.8}", query(e as f64)) + "\n")
        }
        ans
    })
}