use proconio::input;
use std::collections::VecDeque;

fn main() {
    const INF: usize = std::usize::MAX;

    input! {
        n: usize
    }

    /*
    問題で与えられるグラフは n 頂点 n-1 辺の連結グラフと保証されている。
    これは木構造であり、閉路を持たない、かつ
    １辺追加することでただ１つの閉路を形成する。
    よって、そうしてできうる閉路のうち最も長いものの長さが答え。
    その長さは元のグラフの
    　　直径(あらゆる２頂点間の単純パスの最大値) + 1
    に等しい。
    */

    /*
    第 k 要素に「 k 番の頂点と１本の道で繋がっっている頂点全てを
    要素とする Vec 」を持つ Vec を作っておく
    */
    let graph = {
        // ( 0 は空のまま ) 1〜n を使うために len は n+1
        let mut g = vec![vec![]; n+1];
        for _i in 1..=n-1 {
            input! {
                a_i:usize, b_i:usize
            }
            g[a_i].push(b_i);
            g[b_i].push(a_i);
        }
        g
    };

    // 幅優先探索で、start から各頂点までの最短距離を
    // map に直接マッピングしていく
    let dist_map_from = |start: usize| -> Vec<usize> {
        let mut map = vec![INF; n+1];
        // これも 0 + 1〜n

        let mut rem_nodes = VecDeque::with_capacity(n);
        rem_nodes.push_back(start);
        map[start] = 0;

        while !rem_nodes.is_empty() {
            let pos = rem_nodes.pop_front().unwrap();
            for node in &graph[pos] {
                let to = *node;
                if map[to] == INF {
                    map[to] = map[pos] + 1;
                    rem_nodes.push_back(to);
                }
            }
        }

        map
    };

    /*
    以下、問題で与えられたグラフの直径を求める。
    そのために、
    1. 頂点１から (の最短距離が) 最も遠い頂点を探す
    2. その頂点から (の最短距離が) 最も遠い頂点を探すと、
    　　その距離がグラフの直径に等しい
    というアルゴリズムを用いる。
    */

    // 1.
    let dist_map = dist_map_from(1);
    let mut maxdist = 0;
    let mut maxid = 0;
    for i in 1..=n {
        let dist_i = dist_map[i];
        if maxdist < dist_i {
            maxdist = dist_i;
            maxid = i;
        }
    }

    // 2.
    let dist_map = dist_map_from(maxid); // シャドーイング
    let mut maxdist = 0; // シャドーイング
    for i in 1..=n {
        maxdist = maxdist.max(dist_map[i]);
    }

    /*
    １辺追加してできた閉路の長さを出力
    */
    println!("{}", maxdist + 1);
}
