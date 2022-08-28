use std::{ops::Range, cmp};
use proconio::input;


struct Tree(Vec<usize>);
impl Tree {
    fn is_root_index(node_index: usize) -> bool {
        node_index == 0
    }

    fn parent_index_of(node_index: usize) -> usize {
        (node_index - 1) / 2
    }
    fn left_child_index_of(node_index: usize) -> usize {
        (node_index + 1) * 2 - 1
    }
    fn right_child_index_of(node_index: usize) -> usize {
        (node_index + 1 ) * 2
    }

    fn get(&self, index: usize) -> usize {
        self.0[index]
    }
    fn update_value_at(&mut self, update_index: usize, value: usize) {
        self.0[update_index] = value
    }
    /*
                  0
             1           2
          3    4      5      6
         7 8  9 10  11 12

    [ #, #, #, #, #, #, #, #, #, #, #,  #,  # ]
      0  1  2  3  4  5  6  7  8  9  10  11  12
    */
}

struct IndexRange(Range<usize>);
impl IndexRange {
    fn contains(&self, another: &IndexRange) -> bool {
        (self.0.start <= another.0.start) && (another.0.end <= self.0.end)
    }
    fn is_in(&self, another: &IndexRange) -> bool {
        another.contains(self)
    }
    fn is_out_of(&self, another: &IndexRange) -> bool {
        self.0.end <= another.0.start || another.0.end <= self.0.start
    }
    fn intersection(&self, another: &IndexRange) -> Option<IndexRange> {
        if self.is_out_of(another) {
            None
        } else if self.contains(another) {
            Some(IndexRange(another.0.start..another.0.end))
        } else if self.is_in(another) {
            Some(IndexRange(self.0.start..self.0.end))
        } else {
            let (sl, sr, al, ar) = (self.0.start, self.0.end, another.0.start, another.0.end);
            Some(IndexRange(
                if sl < ar {al..sr} else {sl..ar}
            ))
        }
    }
    fn left_half(&self) -> IndexRange {
        let mid = (self.0.start + self.0.end) / 2;
        IndexRange(
            (self.0.start)..mid
        )
    }
    fn right_half(&self) -> IndexRange {
        let mid = (self.0.start + self.0.end) / 2;
        IndexRange(
            mid..(self.0.end)
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
    segment_tree: Tree,
    lazy_tree:    Tree,
} impl /*Lazy*/SegmentTree {
    fn tree_size(&self) -> usize {
        Self::_tree_size_from(self.array_size)
    }
    fn _tree_size_from(array_size: usize) -> usize {
        array_size * 2 - 1
    }

    fn new(array_size_raw: usize) -> /*Lazy*/SegmentTree {
        let array_size = array_size_raw.next_power_of_two();
        let tree_size = Self::_tree_size_from(array_size);

        /*Lazy*/SegmentTree {
            array_size,
            segment_tree: Tree(vec![0; tree_size]),
            lazy_tree:    Tree(vec![0; tree_size]),
        }
    }

    fn range_max(&self, search_range: Range<usize>) -> usize {
        self._range_max(
            &IndexRange(search_range),
            IndexRange(0..self.array_size),
            1
        )
    }
    fn _range_max(&self,
        search_range:            &IndexRange,
        current_searching_range: IndexRange,
        current_index_in_tree:   usize
    ) -> usize {
        match current_searching_range {
            range if range.is_out_of(search_range) => 0,
            range if range.is_in(search_range)     => self.segment_tree.get(current_index_in_tree),
            _ => cmp::max(
                self._range_max(search_range, current_searching_range.left_half(),  Tree::left_child_index_of(current_index_in_tree)),
                self._range_max(search_range, current_searching_range.right_half(), Tree::right_child_index_of(current_index_in_tree))
            )
        }
    }

    fn update(&mut self, update_range: IndexRange, value: usize) {

    }
    fn _update(&mut self, update_index_in_array: usize, value: usize) {
        let mut update_index_in_tree = self.array_size + update_index_in_array;
        while !Tree::is_root_index(update_index_in_tree) {
            self.segment_tree.update_value_at(update_index_in_tree, value);
            update_index_in_tree = Tree::parent_index_of(update_index_in_tree);
        }
    }
}


fn main() {
    input! {w:usize, n:usize}
    let mut segment_tree = SegmentTree::new(w);
    for _ in 0..n {
        input! {l:usize, r:usize}
        
    }
}