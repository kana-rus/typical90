use proconio::input;

fn main() {
    input!{
        n: usize, // <= 100_000
        s: String
    }
    let s: Vec<char> = s.chars().collect();

    let modulo = 1000000007;

    /*
    s ... a t1 t2 c o d e1 r1 e2 r2 (n = 10)
    -> a t1 c o d e1 r1
       a t1 c o d e1 r2
       a t1 c o d e2 r2
       a t2 c o d e1 r1
       a t2 c o d e1 r2
       a t2 c o d e2 r2

    これを

          a  t  c  o  d  e  r
      i   1  2  3  4  5  6  7
    pos
    a 1   1  0  0  0  0  0  0
    t 2   1  1  0  0  0  0  0
    t 3   1  2  0  0  0  0  0
    c 4   1  2  2  0  0  0  0
    o 5   1  2  2  2  0  0  0
    d 6   1  2  2  2  2  0  0
    e 7   1  2  2  2  2  2  0
    r 8   1  2  2  2  2  2  2
    e 9   1  2  2  2  2  4  2
    r 10  1  2  2  2  2  4  6
    
        table[pos][i] = 「 s の 1 〜 pos 文字目までから
                        順番通り i 文字使って、atcoder の
                        i 文字目までを作る方法の総計 」

    となる表 table で整理する。一つ前の状態量を使って効率よく
    次の状態量を求めていく <動的計画法> 。
    */

    let mut table = vec![[0_usize; 1+7]; 1+n];
    // 0 + 1〜7 ,  0 + 1〜n

    table[0][0] = 1; // 0 列目は計算上のヘルパー列

    for pos in 1..=n {
        let index = pos - 1;
        for i in 0..=7 { // 0 列目も継承するため 
            table[pos][i] += table[pos-1][i];

            if s[index]=='a' && i==1 ||
               s[index]=='t' && i==2 ||
               s[index]=='c' && i==3 ||
               s[index]=='o' && i==4 ||
               s[index]=='d' && i==5 ||
               s[index]=='e' && i==6 ||
               s[index]=='r' && i==7 {
                   table[pos][i] += table[pos][i-1];
            }

            table[pos][i] %= modulo;
        }
    }

    println!("{}", table[n][7]);
    
    /* debug
    for pos in 0..=n {
        for i in 0..=7 {
            print!("{} ", table[pos][i]);
            
        }
        println!();
    }
    */
}
