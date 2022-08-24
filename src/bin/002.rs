use proconio::input;

fn main() {
    input! { n: u8 }

    let is_right = |string: &str| -> bool {
        let mut depth = 0;
        for s in string.chars() {
            if s == '(' { depth += 1; }
            if s == ')' { depth -= 1; }
            if depth < 0 { return false; }
        }
        depth == 0
    };

    /*
    ビット全探索。0 から 1<<n - 1 つまり
    100...0 (0がn個) - 1 = 111...1 (1がn-1個) = 2^n -1
    までの 2^n 個の２進数を n 個の (, ) の列に対応させる。
    */
    for i in 0..(1<<n) {
        let mut candidate = String::new();
        // 2進数 i は下の桁から つまり右から見ていく が
        // それに応じて文字列 candidate は左から右へ伸びていく
        for j in 0..n {
            // それを踏まえて辞書順で出力するには...
            if i & 1<<(n-1-j) == 0 { // 2進数 i の j+1 ビット目が 0 なら
                candidate += "(";
            } else {
                candidate += ")";
            }
        }
        if is_right(&candidate) {
            println!("{}", candidate);
        }
    }
}