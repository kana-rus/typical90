use proconio::input;

#[derive(Clone, Copy, Debug)]
enum Color {
    Black,
    White,
    Nil,
} impl Color {
    fn opposite(&self) -> Color {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
            Color::Nil => unreachable!()
        }
    }
    fn is_nil(&self) -> bool {
        match self {
            Color::Nil => true,
            _ => false,
        }
    }
}

struct Tree(Vec<Vec<usize>>);
impl Tree {
    fn build(size: usize) -> Tree {
        let mut tree = vec![vec![]; 1+size];
        for _ in 0..(size-1) {
            input! {a_i:usize, b_i:usize}
            tree[a_i].push(b_i);
            tree[b_i].push(a_i);
        }
        Tree(tree)
    }
    fn dfs_from(&mut self, start_node: usize, current_color: Color, color_map: &mut Vec<Color>) {
        color_map[start_node] = current_color;
        while let Some(node) = self.0[start_node].pop() {
            if color_map[node].is_nil() {
                self.dfs_from(node, current_color.opposite(), color_map)
            }
        }
    }
}

fn main() {
    input! {n: usize}
    let mut tree = Tree::build(n);
    
    let (current_color, mut color_map) = (Color::Black, vec![Color::Nil; 1+n]);
    tree.dfs_from(1, current_color, &mut color_map);

    let mut more_than_half_of_all_nodes = {
        let (mut black_nodes, mut white_nodes) = (vec![], vec![]);
        for i in 1..=n {
            match color_map[i] {
                Color::Black => black_nodes.push(i),
                Color::White => white_nodes.push(i),
                _ => unreachable!()
            }
        }
        if black_nodes.len() > white_nodes.len() {black_nodes} else {white_nodes}
    };
    println!("{}", more_than_half_of_all_nodes.drain(..(n/2)).fold("".to_owned(), |x,y| x+&(y.to_string()+" ")).trim())
}