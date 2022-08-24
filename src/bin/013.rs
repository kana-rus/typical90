use std::collections::VecDeque;
use proconio::input;

const INF: usize = std::usize::MAX;
fn main() {
    input! { n: usize, m: usize }

    let graph = {
        let mut g: Vec<Vec<(usize, usize)>> = vec![vec![]; 1+n];
        for _ in 0..m {
            input! { a_i: usize, b_i: usize, c_i: usize }
            g[a_i].push((b_i, c_i));
            g[b_i].push((a_i, c_i));
        }
        g
    };

    /*
        頂点 k = 1 〜 n のそれぞれについて、「街 1 -> k -> n と移動するコスト」の最小値を求める
        ...-> 街１から任意の街まで、および街ｎから任意の街までの最短距離を事前に求めておき、足し合わせる
    */

    let dijkstra = |start_point: usize| -> Vec<usize> {
        let mut dist_map_from_start_point: Vec<usize> = vec![INF; 1+n];
        let mut queue: VecDeque<usize> = VecDeque::with_capacity(n);

        dist_map_from_start_point[start_point] = 0;
        queue.push_back(start_point);

        while !queue.is_empty() {
            let pos = queue.pop_front().unwrap();
            for (to, cost) in graph[pos].iter() {
                if dist_map_from_start_point[pos] + cost < dist_map_from_start_point[*to] {
                    dist_map_from_start_point[*to] = dist_map_from_start_point[pos] + cost;
                    queue.push_back(*to);
                }
            }
        }

        dist_map_from_start_point
    };

    let (distmap_from_1, distmap_from_n) = (dijkstra(1), dijkstra(n));
    print!("{}", {
        let mut ans = String::new();
        for k in 1..=n {
            ans += &((distmap_from_1[k] + distmap_from_n[k]).to_string() + "\n");
        }
        ans
    })
}