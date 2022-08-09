pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::convert::TryInto;
    use std::collections::HashMap;
    
    let min_value = nums.iter().min().unwrap();
    let new_target = target - 2 * min_value;
    let mut add_nums: Vec<i32> = Vec::new();

    for (_index, item) in nums.iter().enumerate() {
        add_nums.push(item - min_value);
    }


    let mut re: Vec<i32> = Vec::new();

    if new_target % 2 == 0 {
        for (index, item) in add_nums.iter().enumerate() {
            if *item == new_target / 2 {
                re.push(index.try_into().unwrap());
            }
        }
    }

    if re.len() != 2 {
        re.clear();
        let mut backet: Vec<i32> = Vec::new();
        let mut my_map = HashMap::new();
        backet.resize((new_target + 1).try_into().unwrap(), 0);

        for (index, item) in add_nums.iter().enumerate() {
            if *item <= new_target {
                backet[*item as usize] += 1;
                my_map.insert(item, index);
            }
        }

        println!("{:?}", backet);

        for (key, value) in &my_map {
            println!("{}: {}", key, value);
        }

        for (index, item) in backet.iter().enumerate() {
            if *item == 1 && backet[(new_target - index as i32) as usize] == 1 {
                re.push(my_map[&(index as i32)].try_into().unwrap());
                re.push(my_map[&(new_target - index as i32)].try_into().unwrap());
                break;
            }
        }
    }

    return re;
}


pub fn two_sum_test(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;
    let mut backet: HashMap<i32, i32> = HashMap::new();
    for i in 0..nums.len() {
        let j = target - nums[i];
        if backet.contains_key(&j) {
            return [*backet.get(&j).unwrap() as i32, i as i32].to_vec();
        } else {
            backet.insert(nums[i], i as i32);
        }
    }
    return [-1, -1].to_vec();
}

fn main() {
    println!("{:?}", two_sum_test([3, 2, 4].to_vec(), 6));
}