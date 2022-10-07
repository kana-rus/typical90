use std::collections::VecDeque;
use proconio::input;

const INF: usize = std::usize::MAX;
fn main() {
    input! { n: usize, m: usize }

    let graph = {
        let mut graph = vec![vec![]; 1+n];
        for _ in 0..m {
            input! { a_i: usize, b_i: usize, c_i: usize }
            graph[a_i].push((b_i, c_i));
            graph[b_i].push((a_i, c_i));
        }
        graph
    };

    /*
        頂点 k = 1 〜 n のそれぞれについて、「街 1 -> k -> n と移動するコスト」の最小値を求める
        ...-> 街１から任意の街まで、および街ｎから任意の街までの最短距離を事前に求めておき、足し合わせる
    */

    let dijkstra = |start: usize| -> Vec<usize> {
        let mut distmap_from_start: Vec<usize> = vec![INF; 1+n];
        let mut queue: VecDeque<usize> = VecDeque::with_capacity(n);

        distmap_from_start[start] = 0;
        queue.push_back(start);

        while !queue.is_empty() {
            let pos = queue.pop_front().unwrap();
            for (to, cost) in &graph[pos] {
                if distmap_from_start[pos] + cost < distmap_from_start[*to] {
                    distmap_from_start[*to] = distmap_from_start[pos] + cost;
                    queue.push_back(*to)
                }
            }
        }

        distmap_from_start
    };

    let (distmap_from_1, distmap_from_n) = (dijkstra(1), dijkstra(n));
    print!("{}", (1..=n).fold(String::new(), |j, k|
        j + &(distmap_from_1[k] + distmap_from_n[k]).to_string() + "\n"
    ))
}