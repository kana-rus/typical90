use proconio::input;
use std::cmp::max;


// const INF: isize = std::isize::MAX;

fn main() {
    input! {w:usize, n:usize}

    let (min_weight, max_weight, value) = {
        let (mut max, mut min, mut val) = (
            Vec::with_capacity(1+n),
            Vec::with_capacity(1+n),
            Vec::with_capacity(1+n)
        );
        min.push(0); max.push(0); val.push(0); // padding

        for _ in 0..n {
            input! {l:usize, r:usize, v:usize}
            min.push(l); max.push(r); val.push(v)
        }
        (min, max, val)
    };
    let weight_range = |i| min_weight[i]..=max_weight[i];
    
    /* dp[i][j]:
        1 から i 番目までの料理を
        香辛料合計 j [mg] で作ったときに
        可能な価値合計の最大値
    - ans = dp[n][w]
    - i = 0 の行を初期状態 (１つも料理を作っていない状態) として使う
    - i, j ≥ 1 において、dp[i][j] = 0 で
        「１から i 番目までの料理をどの組み合わせで作っても
          香辛料合計 j [mg] にするのは不可能」
      ということを表す
    - k 番目の料理を
      - 作らない: dp[k][j] = dp[k-1][j]
      - 作る: dp[k][j] = max{
          dp[k-1][w-weight] for weight in weight_range[k]
        } + value[k]
    */
    let mut dp = vec![vec![/*-INF*/0; 1+w]; 1+n];
    // dp[0][0] = 0;
    for i in 1..=n {
        for j in 0..=w {
            let without_ith = dp[i-1][j];
            let with_ith = weight_range(i)
                .filter(|&weight| weight <= j)
                .scan(0, |max_value, weight|
                    Some(max(*max_value, dp[i-1][j-weight]))
                )
                .last().unwrap_or(0)
            + value[i];

            dp[i][j] = max(without_ith, with_ith)
        }
    }

    let ans = dp[n][w];
    if ans == 0 {println!("-1")} else {println!("{}", ans)}
}