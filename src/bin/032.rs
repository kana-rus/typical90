use {proconio::input, itertools::Itertools};

const N_LIMIT: usize = 10;
const INF: u16 = std::u16::MAX;

fn main() {
    input! {n:usize /* <= 10 */}
    let times = {
        let mut a = [[0_u16; 1+N_LIMIT]; 1+N_LIMIT];
        for i in 1..=n {
            for j in 1..=n {
                a[i][j] = {input! {time:u16} time}
            }
        } a
    };
    input! {m:usize}
    let bad_terms = {
        let mut bad_terms = [[false; 1+N_LIMIT]; 1+N_LIMIT];
        for _ in 0..m {
            input! {x:usize, y:usize}
            // =========================================================================
            // {x:u8, y:u8} として bad_terms[x as usize][y as usize] 〜　とする方がいいのか？
            // =========================================================================
            bad_terms[x][y] = true;
            bad_terms[y][x] = true;
        } bad_terms
    };

    /*
    input! {
        n: usize,
        a: [[0_u16; n]; n],
        m: usize,
        xy: [(u16, u16); m]
    }

    だと a, xy が Vec になってしまうが、今回は n <= 10 なので積極的にスタックに
    データを置きたい。
    また、こうしてしまうと Vec にインデックスでアクセスしないといけないので読みにくくなる
    気がする
    */

    /* 以下、n <= 10 の制約を活かして n! 通りの走順を全探索する */
    let mut min_sum_of_time = INF;
    'check_running_order: for mut running_order in (1..=n).permutations(n) {
        running_order.insert(0, 0); // padding
        
        let sum_of_time = {
            let mut sum = 0;

            for section in 1..=(n-1) {
                let (runner, next_runner) = (running_order[section], running_order[section+1]);
                if bad_terms[runner][next_runner] {
                    continue 'check_running_order
                }

                sum += times[runner][section]
            }
            sum + times[running_order[n]][n]
        };
        min_sum_of_time = min_sum_of_time.min(sum_of_time)
    }

    if min_sum_of_time==INF {println!("-1")} else {println!("{}", min_sum_of_time)}
}