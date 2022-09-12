use proconio::input;
use std::collections::HashMap;

fn main() {
    input!{
        n:usize, k:usize,
        a:[usize;n]
    }
    let a = [vec![0], a].concat();// padding (好み)

    /*
    数列 s..=e に対して、数列 s..=(e+1) が含む数の種類数は
    - e+1 == one of s..=e     なら同じ
    - e+1 != sny one of s..=e なら１つ多い (...*1)
    のどちらかにしかならない。
    一般に s..=(e+n) が含む数の種類は s..=e が含む種類以上。

    この単調性から、次のアルゴリズムで効率よく解ける。

    s..=x が含む数が K 種類以下になる (...*2) ような max.x を x_s とすると、
    左端が s のときの題意の部分列は、最大で長さ x_s - s + 1 、ということになる。
    次に左端をインクリメントして同じく条件を満たす最大の x を x_{s+1} とすると、
    左端が x+1 なら最大で長さ x_{s+1} - (s+1) + 1 と分かる。

    数列 a_1..=a_N の部分列の左端は当然 a_1 から a_N のいずれかだから、以上の操作を
    s = a_1 から始めて s = a_N まで行えば効率よく答えの候補を挙げることができる。
    *2 の条件チェックをするにあたって、*1 のような感じで毎回そのときの s 以降の数全てと照らし合わせる必要があるので、
    HashMap を使う。この HashSet 周りの操作が logN かかるので、全体で NlogN になる。
    */

    // 以下の s, e は start, end のインデックスを指している。要修正
    let mut max_len = 1;

    let mut already_have = HashMap::new();
    // 2 3 2 4 5 みたいな場合に、最初の２が範囲から出ても３番目の２が残っているので "already have 2."
    // --> HashSet ではなく、HashMap で出てきた回数を count する必要がある
    already_have.insert(a[1], 1);
    let mut e = 1; // 以降、e は「それまでに読み込んだ数のうち最後のもの」みたいな感じで使っている

    for s in 1..=n {
        if s > 1 {
            let count = already_have.get_mut(&a[s-1]).unwrap();
            *count -= 1;
            if count == &0 {already_have.remove(&a[s-1]);}
        }

        loop {
            if e < n && (
                already_have.len() < k
                || (already_have.len() == k && already_have.contains_key(&a[e+1]))
            ) {
                e += 1;
                already_have.entry(a[e])
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            } else {break;}
        }

        max_len = max_len.max(e - s + 1)//(len)
    }

    println!("{}", max_len)
}