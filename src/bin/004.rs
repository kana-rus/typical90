use proconio::input;

fn main() {
    input!{
        h: usize, w: usize,
        a: [[usize; w]; h]
    }

    let mut row_sum = vec![0; h];
    let mut col_sum = vec![0; w];
    for i in 0..h {
        for j in 0..w {
            row_sum[i] += a[i][j];
            col_sum[j] += a[i][j];
        }
    }

    let mut ans = String::new();
    for i in 0..h {
        for j in 0..w {
            let ans_ij = row_sum[i] + col_sum[j] - a[i][j];

            if j>0 { ans += " "; } 
            ans += &ans_ij.to_string();
        }
        ans += "\n"
    }
    print!("{}", ans);
}
