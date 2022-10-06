use proconio::input;

fn main() {
    input!{
        n: usize,
        class_point: [(usize, usize); n],
        q: usize,
        l_r: [(usize, usize); q]
    }

    /*
    1組、2組それぞれについて、「学籍番号 i までの得点の累積和」
    をメモしておく。これにより、「x 番から y 番までの得点の和」を
                    sum[y] - sum[x-1]
    で求められる。
    */
    let (sum1, sum2) = {
        let mut sum1 = vec![0; 1+n];
        let mut sum2 = vec![0; 1+n];

        for i in 1..=n {
            sum1[i] = sum1[i-1];
            sum2[i] = sum2[i-1];
            let (class, point) = class_point[i-1];
            match class {
                1 => sum1[i] += point,
                2 => sum2[i] += point,
                _ => unreachable!()
            }
        }
        (sum1, sum2)
    };

    let mut ans = String::new();
    for (l, r) in l_r {
        ans += &(sum1[r] - sum1[l - 1]).to_string();
        ans += " ";
        ans += &(sum2[r] - sum2[l - 1]).to_string();
        ans += "\n"
    }
    println!("{}", ans);
}
