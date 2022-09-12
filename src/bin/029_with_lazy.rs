/*
    029_normal.rs の続き。
    029_normal.rs では、セグメント木の習得のために可読性を重視して色々と補助関数を
    用意していたが、ここではセグメント木自体にはある程度慣れたという前提でそれらの一部は省略する。

    以下では 029_normal の SegmentTree に lazy という木を追加して LazySegmentTree
    (日本語では「遅延評価セグメント木」) という struct を構成する。
    SegmenTree では配列の指定された範囲の update を、単に一点の update を範囲内の全ての点に順次適用することで実現していたが、
    LazySegmentTree では、
        「ノード x の値が必要になるタイミングまで、ノード x (およびその親たち) の値の更新は準備だけに留める」
    という仕組みにより 範囲 update を O(log(配列長)) で行えるようになっている。
*/
use proconio::input;
use std::{ops::RangeInclusive, cmp::max, fmt::Debug};


const ROOT_POS: usize = 1;

fn left_child_pos(tree_pos: usize) -> usize {
    tree_pos * 2
}
fn right_child_pos(tree_pos: usize) -> usize {
    tree_pos * 2 + 1
}

struct PosRange(RangeInclusive<usize>);
impl PosRange {
    fn is_in(&self, another: &PosRange) -> bool {
        another.0.start() <= self.0.start() && self.0.end() <= another.0.end()
    }
    fn is_out_of(&self, another: &PosRange) -> bool {
        self.0.end() < another.0.start() || another.0.end() < self.0.start()
    }

    fn left_half(&self) -> PosRange {
        let mid = (self.0.start() + self.0.end()) / 2;
        PosRange(*self.0.start()..=mid)
    }
    fn right_half(&self) -> PosRange {
        let mid = (self.0.start() + self.0.end()) / 2 + 1;
        PosRange(mid..=(*self.0.end()))
    }
}

struct LazySegmentTree {
    segments:   Vec<usize>,
    lazy:       Vec<Option<usize>>,
} impl LazySegmentTree {
    fn new(array_size_raw: usize) -> LazySegmentTree {
        let array_size = array_size_raw.next_power_of_two();
        let tree_size = array_size * 2 - 1;

        LazySegmentTree {
            segments: vec![0;    1+tree_size],
            lazy:     vec![None; 1+tree_size],
        }
    }

    fn range_max(&mut self, target_array_range: RangeInclusive<usize>) -> usize {
        let array_size = (self.segments.len() + 1) / 2;

        // =========================================================================================================
        let target_array_range_copy = *target_array_range.start()..=*target_array_range.end();
        let range_max =
        // =========================================================================================================
        self._range_max(
            &PosRange(target_array_range),
            ROOT_POS,  // root から辿る
            PosRange(1..=array_size)
        )
        // =========================================================================================================
        ;
        println!("range_max of {:?} is {} :\n{:?}\n", target_array_range_copy, range_max, self);
        range_max
        // =========================================================================================================
    }
    fn _range_max(&mut self,
        target_array_range:  &PosRange,
        tree_pos:            usize,
        dealing_array_range: PosRange  // tree_pos と array_size から計算できるが入れておく
    ) -> usize {
        match dealing_array_range {
            r if r.is_in(target_array_range)     => self.segments[tree_pos],
            r if r.is_out_of(target_array_range) => 0,
            _ => {
                if self.lazy[tree_pos].is_some() {self._propagate(tree_pos, &dealing_array_range, &dealing_array_range)}
                max(
                    self._range_max(target_array_range, left_child_pos(tree_pos),  dealing_array_range.left_half()),
                    self._range_max(target_array_range, right_child_pos(tree_pos), dealing_array_range.right_half())
                )
            }
        }
    }

    fn update(&mut self, updating_range: RangeInclusive<usize>, new_value: usize) {
        let array_size = (self.segments.len() + 1) / 2;
        // =========================================================================================================
        let updating_range_copy /* for debug */ = *updating_range.start()..=*updating_range.end();
        // =========================================================================================================

        self._update(
            &PosRange(updating_range),
            Some(new_value),
            ROOT_POS,  // root から辿る
            PosRange(1..=array_size)
        )
        // =========================================================================================================
        ;
        println!("updated range {:?} to {new_value} :\n{:?}\n", updating_range_copy, self)
        // =========================================================================================================
    }
    fn _update(&mut self,
        target_array_range:  &PosRange,
        new_value:           Option<usize>,
        tree_pos:            usize,
        dealing_array_range: PosRange  // tree_pos と array_size から計算できるが入れておく
    ) {
        if ! dealing_array_range.is_out_of(&target_array_range) {
            if new_value.is_some() {self.lazy[tree_pos] = new_value}

            if dealing_array_range.is_in(&target_array_range) {
                if new_value.is_some() {self.segments[tree_pos] = new_value.unwrap()}
                // 自ノードだけを実際に更新して抜ける (子ノードは必要になるまで更新しない)
            } else {
                self._propagate(tree_pos, target_array_range, &dealing_array_range);
                // target_range に入ってる 部分と入ってない部分がある場合、子孫ノードに伝搬する
                self.segments[tree_pos] = max(
                    self.segments[left_child_pos(tree_pos)],
                    self.segments[right_child_pos(tree_pos)]
                )
            }
        }
    }

    fn _propagate(&mut self,
        tree_pos:            usize,
        target_array_range:  &PosRange,
        dealing_array_range: &PosRange
    ) {
        let lazy_val = self.lazy[tree_pos].take();
        self._update(target_array_range, lazy_val, left_child_pos(tree_pos),  dealing_array_range.left_half());
        self._update(target_array_range, lazy_val, right_child_pos(tree_pos), dealing_array_range.right_half());
    }
}
// =========================================================================================================
impl Debug for LazySegmentTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let log2 = |mut pow_of_2: usize| {
            let mut pow = 0;
            loop {
                pow_of_2 /= 2; pow += 1;
                if pow_of_2 == 1 {break;}
            }
            pow
        };
        let tree_size = self.segments.len();

        write!(f, "{}", {
            let (mut indent, mut space) = (0, 1);

            let line_num = log2(tree_size + 1);
            let mut lines = vec![String::new(); line_num];

            for l in 0..line_num { let line_idx = line_num - l - 1;
                lines[line_idx] += &" ".repeat(indent);
                lines[line_idx] += self.segments[2_usize.pow(line_idx as u32)..=2_usize.pow((line_idx+1) as u32)-1]
                    .iter().fold("".to_owned(), |x,y| x + &y.to_string() + &" ".repeat(space)).trim();

                lines[line_idx] += &(" ".repeat(indent) + "  ");

                lines[line_idx] += &" ".repeat(indent);
                lines[line_idx] += self.lazy[2_usize.pow(line_idx as u32)..=2_usize.pow((line_idx+1) as u32)-1]
                    .iter().map(|x| if x.is_none() {"N".to_owned()} else {x.unwrap().to_string()})
                    .fold("".to_owned(), |x,y| x + &y + &" ".repeat(space)).trim();

                indent += 2_usize.pow(l as u32);
                space  += 2_usize.pow((l + 1) as u32);
            }
            lines.join("\n")
        })
    }
}
// =========================================================================================================

fn main() {
    input! {w:usize, n:usize}

    let mut segment_tree = LazySegmentTree::new(1+w);
    let mut ans = String::new();

    for _ in 0..n {
        input! {l:usize, r:usize}
        let max_height_in_l_r = segment_tree.range_max(l..=r);

        let next_height = max_height_in_l_r + 1;
        ans += &(next_height.to_string() + "\n");
        segment_tree.update(l..=r, next_height);
    }
    print!("{}", ans);
}