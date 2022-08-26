use proconio::input;

struct GraphForth(Vec<Vec<usize>>);
impl GraphForth {
    fn dfs_from(&mut self, start_node: usize, visit_map: &mut Vec<bool>, order: &mut Vec<usize>) {
        visit_map[start_node] = true;
        while let Some(node) = self.0[start_node].pop() {
            if !visit_map[node] {self.dfs_from(node, visit_map, order)}
        }
        order.push(start_node)
    }
}
struct GraphBack(Vec<Vec<usize>>);
impl GraphBack {
    fn dfs_from(&mut self, start_node: usize, visit_map: &mut Vec<bool>, count: &mut usize) {
        visit_map[start_node] = true;
        *count += 1;
        while let Some(node) = self.0[start_node].pop() {
            if !visit_map[node] {self.dfs_from(node, visit_map, count)}
        }
    }
}

struct Graphs((usize, usize), GraphForth, GraphBack);
impl Graphs {
    fn nodes_num(&self) -> usize {(self.0).0}

    fn build(n: usize, m: usize) -> Graphs {
        let (mut gf, mut gb) = (vec![vec![]; 1+n], vec![vec![]; 1+n]);
        for _ in 0..m {
            input! { a_i: usize, b_i: usize }
            gf[a_i].push(b_i);
            gb[b_i].push(a_i);
        }
        Graphs((n,m), GraphForth(gf), GraphBack(gb))
    }

    fn dfs_forth(&mut self) -> Vec<usize> {
        let (mut visit_map, mut order) = (vec![false; 1+self.nodes_num()], vec![]);

        for node in 1..=self.nodes_num() {
            if !visit_map[node] {self.1.dfs_from(node, &mut visit_map, &mut order)}
        }
        order
    }
    fn dfs_back(&mut self, order: Vec<usize>) -> usize {
        let mut ans = 0;
        let mut visit_map = vec![false; 1+self.nodes_num()];
        for node in order {
            if !visit_map[node] {
                let mut count = 0;
                self.2.dfs_from(node, &mut visit_map, &mut count);
                ans += count * (count-1) / 2;
            }
        }
        ans
    }
}

/*
    有向グラフは、とりあえず
    - 順方向のグラフ (インデックス i の要素に「 i 番のノードから直接行けるノード番号の Vec 」をもつ Vec)
    と
    - 逆方向のグラフ (インデックス i の要素に「 i 番のノードに直接行けるノード番号の Vec 」をもつ Vec)
    をそれぞれ持って表現することにする。

    グラフ全体を強連結成分分解するとよい。分解後の各成分において「互いに行き来可能な２頂点」は (成分の頂点数) C 2 通りあり、
    同じ成分に属さない頂点間でこの関係が成り立つことは明らかにないので、これで解けている。

    具体的に、高速に強連結成分分解するには以下のアルゴリズムを用いる。
    1. 順方向グラフを深さ優先探索して帰りがけ順を記録する。
    2. 逆方向グラフを、１で記録した順番の逆順で深さ優先探索する。このとき必ず各強連結成分ごとに探索が切れるようになっているので、
       この探索に合わせて各成分の頂点数を記録し、ans に (頂点数) C 2 を加える。
*/

fn main() {
    input! { n: usize, m: usize }
    let mut graphs = Graphs::build(n, m);

    let mut order = graphs.dfs_forth();
    order.reverse();
    println!("{}", graphs.dfs_back(order))
}