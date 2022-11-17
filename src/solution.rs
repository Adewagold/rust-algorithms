use std::collections::HashMap;

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        for i in 0..nums.len(){
            for j in 0..nums.len(){
                if i == j {
                    continue;
                }
                if nums.get(i).unwrap() + nums.get(j).unwrap() == target{
                    result.push(i as i32);
                    result.push(j as i32); 
                    return result;
                }
            }
        }
        result
    }


    pub fn two_sum_map(nums: Vec<i32>, target:i32) -> Vec<i32>{
        let mut result: Vec<i32> = Vec::new();
        let mut nums_map: HashMap<i32, i32> = HashMap::new();
        for (i,num) in nums.into_iter().enumerate(){
            let target_value = target - num;
            let is_target = nums_map.get(&target_value);
            if is_target != None {
                let last_index = is_target.unwrap().abs();
                result.push(last_index);
                result.push(i as i32);
                return result;
            }
            nums_map.insert(num, i as i32);
        }
        unreachable!()
    }

    pub fn greatest_common_divisor(mut n:u64, mut m:u64) -> u64 {
        assert!(n!=0 && m!=0);
        while m != 0 {
            if m < n {
                let t = m;
                m = n;
                n = t;
            }
            
            m = m % n;
        }
        n
    }