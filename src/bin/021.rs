use proconio::input;

struct Graphs(Vec<Vec<usize>>, Vec<Vec<usize>>);
impl Graphs {
    fn dfs_forth_from(&mut self, start_node: usize, visit_map: &mut Vec<bool>, order: &mut Vec<usize>) {
        visit_map[start_node] = true;
        while let Some(node) = self.0[start_node].pop() {
            if !visit_map[node] {self.dfs_forth_from(node, visit_map, order)}
        }
        order.push(start_node)
    }
    fn dfs_back_from(&mut self, start_node: usize, visit_map: &mut Vec<bool>, count: &mut usize) {
        visit_map[start_node] = true;
        *count += 1;
        while let Some(node) = self.1[start_node].pop() {
            if !visit_map[node] {self.dfs_back_from(node, visit_map, count)}
        }
    }
}

fn main() {
    input! { n: usize, m: usize }

    let mut graphs = {
        let (mut gf, mut gb) = (vec![vec![]; 1+n], vec![vec![]; 1+n]);
        for _ in 0..m {
            input! { a_i: usize, b_i: usize }
            gf[a_i].push(b_i);
            gb[b_i].push(a_i);
        }
        Graphs(gf, gb)
    };

    let (mut visit_map, mut order) = (vec![false; 1+n], vec![]);
    for node in 1..=n {
        if !visit_map[node] {graphs.dfs_forth_from(node, &mut visit_map, &mut order)}
    }

    let mut ans = 0;
    let (mut visit_map, order) = (vec![false; 1+n], {order.reverse(); order});
    for node in order {
        if !visit_map[node] {
            let mut count = 0;
            graphs.dfs_back_from(node, &mut visit_map, &mut count);
            ans += count*(count-1)/2
        }
    }

    println!("{}", ans)
}