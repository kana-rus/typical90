use proconio::input;

fn main() {
    input! {
        n: usize, l:usize,
        k: usize,
        a: [usize; n]
    }

    /*
    「 長さ m 以上のピースに分割することによって
    ピース数を k+1 個以上にすることが可能か 」を調べる。
    ( 当然、できるだけ多くのピースに分割したいので、
    端から貪欲に長さ m 以上のピースを切り取っていく。)
    */
    // これは「 答えが m 以上かどうか 」を調べていることになる。
    let ans_is_more_than = |m: usize| -> bool {
        let mut pieces: usize = 1;
        let mut pre_cut: usize = 0;
        for i in 0..n {
            if a[i] - pre_cut >= m && l - a[i] >= m {
                pre_cut = a[i];
                pieces += 1;
            }
        }
        pieces >= k+1
    };

    /*
    これで、ある長さ m に対し、答えが m 以上かどうかを返せるようになった。
    これを使って二部探索で答えを探す。
    */
    // m はもちろん 0 以上 l 以下
    let mut left = 0;
    let mut right = l;
    while right - left > 1 {
        let mid = (left + right ) / 2;
        if ans_is_more_than(mid) { // 答えが mid 「 以上 」なら
            left = mid;
        } else { // 答えが mid 「 未満 」なら
            right = mid;
        }
    }
    // while を抜けて left right が 1 差で並んだとき
    // 答えは「 left 以上 right 未満 」

    // println!("{left}");
    // とすると atcoder の Rust が 1.42.0 のため CompileError
    println!("{}", left)
}