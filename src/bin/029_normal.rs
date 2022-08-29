use std::{ops::RangeInclusive, cmp, fmt::Debug};
use proconio::input;



const ROOT_IDX: usize = 1;

type CBT = CompleteBinaryTree;
struct CompleteBinaryTree(Vec<usize>);
impl CompleteBinaryTree {
    fn parent_pos_of(node_pos: usize) -> usize {
        node_pos / 2
    }
    fn left_child_pos_of(node_pos: usize) -> usize {
        node_pos * 2
    }
    fn right_child_pos_of(node_pos: usize) -> usize {
        node_pos * 2 + 1
    }

    fn get(&self, pos: usize) -> usize {
        self.0[pos]
    }
    fn update_value_at(&mut self, update_pos: usize, value: usize) {
        self.0[update_pos] = value
    }
    /*
                  1
             2           3
          4    5      6      7
         8 9  10 11 12 13

    [ #, #, #, #, #, #, #, #, #, #,  #,  #,  # ]
      1  2  3  4  5  6  7  8  9  10  11  12  13
    */
}

struct IdxRange(RangeInclusive<usize>);
impl IdxRange {
    fn contains(&self, another: &IdxRange) -> bool {
        (self.0.start() <= another.0.start()) && (another.0.end() <= self.0.end())
    }
    fn is_in(&self, another: &IdxRange) -> bool {
        another.contains(self)
    }
    fn is_out_of(&self, another: &IdxRange) -> bool {
        self.0.end() < another.0.start() || another.0.end() < self.0.start()
    }
    fn left_half(&self) -> IdxRange {
        let mid = (self.0.start() + self.0.end()) / 2;
        IdxRange(
            (*self.0.start())..=mid
        )
    }
    fn right_half(&self) -> IdxRange {
        let mid = (self.0.start() + self.0.end()) / 2;
        IdxRange(
            (mid + 1)..=(*self.0.end())
        )
    }
}


struct /*Lazy*/SegmentTree {  // for range_max //
    /*
                  #
             #          #
          #    #     #     #
         $ $  $ $   $ $   $ $
         | |  | |   | |   | |
         | |  | |   | |   | |
       [ $,$, $,$,  $,$,  $,$ ]
    */
    array_size:   usize,
    segment_tree: CompleteBinaryTree,
    // lazy_tree:    CompleteBinaryTree,
} impl /*Lazy*/SegmentTree {
    fn new(array_size_raw: usize) -> /*Lazy*/SegmentTree {
        let array_size = array_size_raw.next_power_of_two();
        let tree_size = array_size * 2 - 1;

        /*Lazy*/SegmentTree {
            array_size,
            segment_tree: CompleteBinaryTree(vec![0; 1+tree_size]),
            // lazy_tree:    CompleteBinaryTree(vec![0; 1+tree_size]),
        }
    }
    fn _tree_size(&self) -> usize {
        self.array_size * 2 - 1
    }

    fn range_max(&self, search_range: RangeInclusive<usize>) -> usize {
        self._range_max(
            &IdxRange(search_range),
            ROOT_IDX,
            IdxRange(ROOT_IDX..=self.array_size)
        )
    }
    fn _range_max(&self,
        search_range:            &IdxRange,
        current_pos_in_tree:     usize,
        current_searching_range: IdxRange, // これは current_searching_pos_in_tree と self.array_size に依存して決まる
                                              // が、引数として持つ方が速い
    ) -> usize {
        match current_searching_range {
            range if range.is_out_of(search_range) => 0,
            range if range.is_in(search_range)     => self.segment_tree.get(current_pos_in_tree),
            _ => cmp::max(
                self._range_max(search_range, CBT::left_child_pos_of(current_pos_in_tree),  current_searching_range.left_half()),
                self._range_max(search_range, CBT::right_child_pos_of(current_pos_in_tree), current_searching_range.right_half())
            )
        }
    }

    fn update(&mut self, updating_pos_in_array: usize, new_value: usize) {
        let mut updating_pos_in_tree = (self._tree_size() / 2) + updating_pos_in_array;
        self.segment_tree.update_value_at(updating_pos_in_tree, new_value);
        // =============================================================================================
        // println!("value at pos({}){}: updated", updating_pos_in_tree, if updating_pos_in_tree < 10 {" "} else {""});
        // println!("{:?}", self.segment_tree.0);
        // =============================================================================================

        while updating_pos_in_tree > ROOT_IDX {
            updating_pos_in_tree = CBT::parent_pos_of(updating_pos_in_tree);
            let old_value = self.segment_tree.get(updating_pos_in_tree);

            if new_value <= old_value {
                // =============================================================================================
                // println!("-- breaked at pos({})", updating_pos_in_tree);
                // =============================================================================================
                break;
            }

            self.segment_tree.update_value_at(updating_pos_in_tree, new_value);
            // =============================================================================================
            // println!("value at pos({}){}: updated", updating_pos_in_tree, if updating_pos_in_tree < 10 {" "} else {""});
            // println!("{:?}", self.segment_tree.0);
            // =============================================================================================
        }
    }
}
impl Debug for SegmentTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let log2 = |mut pow_of_2: usize| {
            let mut pow = 0;
            loop {
                pow_of_2 /= 2; pow += 1;
                if pow_of_2 == 1 {break;}
            }
            pow
        };
        let tree_size = self.segment_tree.0.len();

        write!(f, "{}", {
            let (mut indent, mut space) = (0, 1);

            let line_num = log2(tree_size + 1);
            let mut lines = vec![String::new(); line_num];

            for l in 0..line_num { let line_idx = line_num - l - 1;
                lines[line_idx] += &" ".repeat(indent);
                lines[line_idx] += self.segment_tree.0[2_usize.pow(line_idx as u32)..=2_usize.pow((line_idx+1) as u32)-1]
                    .iter().fold("".to_owned(), |x,y| x + &y.to_string() + &" ".repeat(space)).trim();

                indent += 2_usize.pow(l as u32);
                space  += 2_usize.pow((l + 1) as u32);
            }
            lines.join("\n")
            /*
               1
             2   3
            4 5 6 7

                   1
               2       3
             4   5   6   7
            8 9 9 9 0 0 0 0
            */
        })
    }
}


fn main() {
    input! {w:usize, n:usize}
    let (mut segment_tree, mut ans) = (SegmentTree::new(1+w), String::new());

    for _ in 0..n {
        input! {l:usize, r:usize}
        let max_height_in_l_r = segment_tree.range_max(l..=r);
        let next_height = max_height_in_l_r + 1;

        ans += &(next_height.to_string() + "\n");
        for pos in l..=r {
            segment_tree.update(pos, next_height)
        }
        // =============================================================================================
        // println!("{:?}\n", segment_tree)
        // =============================================================================================
    }

    print!("{}", ans);
}


/*
    以上 SegmentTree による実装だが、これでは update の効率が悪いために TLE する。
    そこで、値の更新を遅延するというアイデアでこれを解決する。
    (029_lazy.rs に続く)
*/