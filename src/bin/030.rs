use proconio::input;

fn main() {
    input! {n:usize, k:usize}

    let mut prime_factors_num_map = vec![0; 2+n];
    for i in 2..=n {
        if prime_factors_num_map[i] == 0 {
            for j in 1..=(n / i) {
                prime_factors_num_map[j * i] += 1
            }
        }
    }

    /*
    ex) n = 9, k = 2
          2 3 4 5 6 7 8 9
        [ 0,0,0,0,0,0,0,0 ]
    i=2 [ 1,0,1,0,1,0,1,0 ]
      3 [ 1,1,1,0,2,0,1,1 ]
      4 skip because map[4] = 1 > 0
      5 [ 1,1,1,1,2,0,1,1 ]
      6 skip because map[6] = 2 > 0
      7 [ 1,1,1,1,2,1,1,1 ]
      8 skip because map[8] = 1 > 0
      9 skip because map[9] = 1 > 0

    要するに、map[i] = 0 という条件がエラトステネスの篩の原理で
    「i が素数」と同値になり、その素数 i の倍数のところに +1 されるので、
    これを 2 以上 n 以下の i 全てで実行することで効率よく ( O(n * log(log(n))) で)
    2 以上 n 以下の整数全てについて素因数の種類を列挙したリストができる。
    */

    println!("{}", prime_factors_num_map.iter().fold(0, |a,b| if b>=&k {a+1} else {a}))
}