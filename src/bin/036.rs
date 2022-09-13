use proconio::input;
use std::cmp::{max, min};

fn main() {
    input! {n:usize, q:usize}

    /*
    一見遠回りだが、S := {(xi,yi) | 1 ≤ i ≤ n} の各点に対して

        (x, y) |-> (X, Y) = (x-y, x+y)

    つまり √2 • e^(πi/4) による回転拡大をかけた集合 T を考える。
    ( xi = (Xi+Yi)/2, yi = (Yi-Xi)/2 )

    このとき、S の２元 (xi,yi), (xj,yj) のマンハッタン距離を
    対応する T の元で表すと

        d((xi,yi),(xj,yj))
      = |xi-xj| + |yi-yj|
      = |(Xi+Yi)/2 - (Xj+Yj)/2| + |(Yi-Xi)/2 - (Yj-Xj)/2|
      = |(Xi-Xj)/2 + (Yi-Yj)/2| + |(Yi-Yj)/2 - (Xi-Xj)/2|
    
    [第１項] [第２項]
       ≥0      ≥0   => Yi-Yj     in Yi-Yj ≥ 0 and Yi-Yj ≥ Xi-Xj
       ≥0      <0   => Xi-Xj     in Xi-Xj > 0 and Xi-Xj > Yi-Yj
       <0      ≥0   => - (Xi-Xj) in Xi-Xj < 0 and Xi-Xj < Yi-Yj
       <0      <0   => - (Yi-Yj) in Yi-Yj < 0 and Yi-Yj < Xi-Xj

      = max{ |Xi-Xj|, |Yi-Yj| } //

    よって、任意の (a,b) \in S について, (a,b) |-> (A,B) とおくと

        max{ d((x1,y1),(a,b)), d((x2,y2),(a,b)), ..., d((xn,yn),(a,b)) }
      = max{ max{ |X1-A|,|Y1-B| }, max{ |X2-A|,|Y2-B| }, ..., max{ |Xn-A|,|Yn-B| } }
      = max{ |X1-A|,|Y1-B|,|X2-A|,|Y2-B|, ..., |Xn-A|,|Yn-B| }
      = max{
          max{ |X1-A|, |X2-A|, ..., |Xn-A| },
          max{ |Y1-B|, |Y2-B|, ..., |Yn-B| }
        }
      = max{
          max{ |X_max - A|, |X_min - A| },
          max{ |Y_max - A|, |Y_min - B| }
        }
      = max{ |X_max - A|, |X_min - A|, |Y_max - A|, |Y_min - B| }

    が成り立ち、回転拡大操作は最初に１回やるだけでよく、X, Y の max, min もそのときに決まる
    ので、素朴にやる ( O(NQ) ) よりかなり計算量を減らせる ( O(N+Q) )
    */

    let xy = {
        let mut xy = Vec::with_capacity(1+n);
        xy.push((0,0)); // padding
        
        for _ in 0..n {
            input! {xi:isize, yi:isize}
            xy.push((xi-yi, xi+yi))
        } xy
    };
    let ((x_max, y_max), (x_min, y_min)) = xy.iter().fold(
        (xy[1], xy[1]), |
            ((p, q), (r, s)), (x, y)
        | ((max(p, *x), max(q, *y)), (min(r, *x), min(s, *y)))
    );

    let mut ans = String::new();
    for _ in 0..q {
        input! {query:usize}
        let (a, b) = xy[query];

        ans += &format!("{}\n", max(
            max((x_max - a).abs(), (y_max - b).abs()),
            max((x_min - a).abs(), (y_min - b).abs())
        ))
    }
    print!("{}", ans)
}