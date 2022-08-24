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
        let mut s1 = vec![0; 1+n];
        let mut s2 = vec![0; 1+n];

        for i in 1..=n {
            s1[i] = s1[i-1];
            s2[i] = s2[i-1];
            let data = class_point[i-1];
            if data.0 == 1 {
                s1[i] += data.1;
            } else {
                s2[i] += data.1;
            }
        }

        (s1, s2)
    };

    let mut ans = String::new();
    for query in l_r {
        ans += &(sum1[query.1] - sum1[query.0 - 1]).to_string();
        ans += " ";
        ans += &(sum2[query.1] - sum2[query.0 - 1]).to_string();
        ans += "\n"
    }
    println!("{}", ans);
}
