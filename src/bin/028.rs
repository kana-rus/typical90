use std::{vec, fmt::Display};
use proconio::input;

/*
座標は交差点 (0〜N)、面積はマス目 (1個め〜N個め) で数えているので１つずれることに注意
*/

/*
fn main() {
    input! {n:usize}

    let mut field = vec![vec![0;1+1000];1+1000];
    for _ in 0..n {
        input! {lx:usize, ly:usize, rx:usize, ry:usize}
        for i in (1+ly)..=ry {
            for j in (1+lx)..=rx {
                field[i][j] += 1
            }
        }
    }

    let mut ans = vec![0;1+n];
    for i in 1..=1000 {
        for j in 1..=1000 {
            ans[field[i][j]] += 1
        }
    }
    print!("{}", ans[1..=n].iter().fold("".to_string(), |x,y| x + &y.to_string() + "\n"))
}

としたくなるところだが、これでは O(NWH) (ここでは W = H = 1000) となり TLE する.
そこで、
    紙の各マスに +1 する: O(N)
という操作を、
    1. 紙の四隅 (左上のマスと、右下のマスの1つ右下) を
        +1 ---- -1
        |       |
        |       |
        -1 ---- +1
       のように +-1 する: O(1)
       (プログラムで直接扱う配列に合わせて、座標は
            0 ----------------> x
            |
            |
            |
            ↓
            y
        ととっている)
    2. 縦横に累積和をとる: O(N)
と分割する. このとき、1, 2 には以下の性質がある：
    *重要* 複数の 1, 2 を繰り返す操作の結果は、
    「その 1 のみを全て行ってから、全ての紙を含む領域に対して一度だけ 2 を行う」
    という操作の結果と同じである
よって、
    +-1 のプロットだけを全てやってから、1000 * 1000 の field 全体で縦横に累積和をとる
という手順をとることで、元の O(NWH) を O(N + WH) に落とすことができる。
これを imos法 という。
*/

const SIZE: usize = 1000;

struct Field(Vec<Vec<isize>>);
impl Field {
    fn new() -> Field {
        Field(vec![
            vec![0; 1+SIZE]
        ; 1+SIZE])
    }
    fn accumulate(&mut self,
        lx:usize, ly:usize,  // この field では視覚的に左上
        rx:usize, ry:usize   // この field では視覚的に右下
    ) {
        self.0[ly][lx] += 1;
        self.0[ly][rx] -= 1;
        self.0[ry][lx] -= 1;
        self.0[ry][rx] += 1;
        // 座標の値のインデックス (0〜) で accumulate することで、
        // のちに calc したときにちょうど紙があるマス目のインデックス (0〜) のところ全てに +1 されることになる
    }
    fn calc(&mut self) {
        // 縦に累積和
        for i in 0..SIZE {
            for j in 0..SIZE {
                self.0[i+1][j] += self.0[i][j]
            }
        }
        // 横に累積和
        for i in 0..SIZE {
            for j in 0..SIZE {
                self.0[i][j+1] += self.0[i][j]
            }
        }
    }
    fn total(&self, k_max: usize) -> Vec<usize> {
        let mut ans = vec![0; 1+k_max];
        for i in 0..SIZE {
            for j in 0..SIZE {
                let k /* (かさなり枚数) */ = self.0[i][j] as usize;
                ans[k] += 1
            }
        }
        ans
    }
}
impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
            self.0.iter().map(|v|
                v.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" ")
            ).collect::<Vec<_>>().join("\n")
        )
    }
}

fn main() {
    input! {n:usize}
    let mut field = Field::new();

    for _ in 0..n {
        input! {lx:usize, ly:usize, rx:usize, ry:usize}
        field.accumulate(lx, ly, rx, ry)
    }

    field.calc();

    let ans = field.total(n);
    print!("{}", ans[1..].iter().fold("".to_owned(), |x,y| x + &y.to_string() + "\n"))
}