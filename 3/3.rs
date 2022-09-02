pub fn length_of_longest_substring(s: String) -> i32 {
    use std::collections::HashMap;

    if s.len() == 0 {
        return 0;
    }

    let mut map:HashMap<char, i32> = HashMap::with_capacity(s.len());
    let s = s.chars().collect::<Vec<_>>();
    let mut max = 0;
    let mut left = 0;

    s.iter().enumerate().for_each(|(index, char_)|{
        if let Some(last) = map.get(char_) {
            left = std::cmp::max(left, *last);
        }
        max = std::cmp::max(max, index as i32 - left + 1);
        map.insert(*char_, index as i32 + 1);
    });
    return max as i32;
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    // println!("??");
    let s = String::from("afebcfefx");
    assert_eq!(5, length_of_longest_substring(s));
}
