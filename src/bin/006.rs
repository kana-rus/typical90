use proconio::input;

fn alphabet(index: usize) -> char {
    (index as u8 + 96) as char
}

fn main() {
    input!{
        n: usize, k:usize,
        s: String
    }
    let s: Vec<char> = s.chars().collect();
    let inf = n+1;

    /*  1  2  3  4  5  6  7
    S:  h  o  g  e  e  g  h

    a   -  -  -  -  -  -  -
    b   -  -  -  -  -  -  -
    c   -  -  -  -  -  -  -
    d   -  -  -  -  -  -  -
    e   4  4  4  4  5  -  -
    f   -  -  -  -  -  -  -
    g   3  3  3  5  5  5  -
    h   1  1  1  1  1  1  7
    :
    :
    o   2  2  -  -  -  -  -
    :
    :
    y   -  -  -  -  -  -  -
    z   -  -  -  -  -  -  -

    のように、i 行 j 列に「 S の j 文字目以右で
    i 番目のアルファベットが初登場するのは S の何文字目か 」
    が入った表を作る (登場しない場合は S の長さより大きい数
    を入れておけばよい)。
    後ろから累積的に計算して作れる。
    */
    let list = {
        let mut li = vec![vec![inf; 1+n+1]; 1+26];
        // 0 + 1〜n + n+1,  0 + 1〜26
        for i in 1..=26 {
            for back in 1..=n { let j = n - back + 1; // S の j 文字目
                if s[j-1] == alphabet(i) { // インデックス j-1
                    li[i][j] = j;
                } else {
                    li[i][j] = li[i][j+1]; // このために n+2 列目を用意した
                }
            }
        }
        li
    };

    let mut ans = String::new();
    /*
    list を使って、S を左から見ていき「 次に ans に採用できる文字の中で
    辞書順最小のもの 」貪欲に ans に足していく

     S のある文字 x を次に ans に採用できる
    ＜＝＞ (その時点の ans の文字数) + (x 以右の全ての文字数) >= k
    ＜＝＞ x が S の j 文字目であり i 番目のアルファベットである
    　　　 とすると
    　　　     ans.len() + (1 + n - list[i][j]) >= k

    となるので、「 j (1〜n) ごとに list の値を上から (= アルファベット順に)
    見ていって、この条件を最初に満たした文字を採用する 」という操作を、
    ans が k 文字になるまで繰り返す
    */

    /*
    list の j' 列目を調べた結果 S の j 文字目にある文字を採用したとする。
    この場合、その後 list の j'+1 〜 j 列目を調べることは無駄なので、
    「 今 list の何列目を調べているか (= j') 」を current_pos という変数で管理し、
    S の j 文字目を採用するごとに current_pos を j + 1 に更新する
    */
    let mut current_pos = 1;
    
    for process_ans_len in 0..=k-1 {
        for i in 1..=26 {
            let j = list[i][current_pos];
            if process_ans_len + (1 + n - j) >= k {
                ans += &alphabet(i).to_string();
                current_pos = j + 1;
                break;
            }
        }
    }
    
    println!("{}", ans);
}
