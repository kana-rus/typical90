use {std::collections::HashSet, proconio::input};

fn main() {
    input! {n:usize}
    let (mut map, mut ans) = (HashSet::new(), String::new());
    for i in 1..=n {
        if map.insert({input!{s_i:String} s_i}) {
            ans += &(i.to_string()+"\n")
        }
    }
    print!("{}", ans)
}