use proconio::input;

fn main() {
    /*
    input! {
        n: usize,
        mut coins: [usize; 3]
    }
    coins.sort_unstable();

    let max_coin = coins[2];
    let mid_coin = coins[1];
    let min_coin = coins[0];

    let max_coin_lim = n / max_coin;
    'search:
    for i in 0..=max_coin_lim {
        let max_coin_num = max_coin_lim - i;
        let remain_price = n - max_coin * max_coin_num;

        let mid_coin_lim = remain_price / mid_coin;
        for j in 0..=mid_coin_lim {
            let mid_coin_num = mid_coin_lim - j;
            let remain_price = remain_price - mid_coin * mid_coin_num;

            if remain_price % min_coin == 0 {
                let min_coin_num = remain_price / min_coin;
                println!("{max_coin_num}, {mid_coin_num}, {min_coin_num}");
                println!("{}", max_coin_num + mid_coin_num + min_coin_num);
                break 'search;
            }
        }
    }

    で max_coin から大きい順に貪欲に使っていくのが速くていいと思っていたが、
    それで求まる組み合わせ (max_coin_num を最大化した組) において、
    倍数の噛み合わせの問題で
    - mid_coin_num が極端に少なく
    - min_coin_num が極端に多く
    なっていることがある。この場合に、max_coin_num を減らしてでも倍数の噛み合わせを
    よくすることで、さっきの組み合わせより max_coin_num を少なくするかわりに
    min_coin_num の大部分を mid_coin_num に寄せて、全体としては枚数を減らせる
    ことがある。これを検知するために全探索を採用する。
    */

    input! {
        n: isize,
        a: isize, b: isize, c: isize
    }

    let mut candidate = 9999;

    for i in 0..=9999 {
        for j in 0..=9999-i {
            let rem = n - a*i - b*j;
            if rem >= 0 && rem % c == 0 {
                let coin_num = i + j + (rem / c);
                if coin_num < candidate {
                    candidate = coin_num;
                }
            }
        }
    }

    println!("{}", candidate);
}