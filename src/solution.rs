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

    #[test]
    fn test_greates_common_divisor(){
        assert_eq!(greatest_common_divisor(14,15),1);
        assert_eq!(greatest_common_divisor(2 * 3 * 5 * 11 * 17,3 * 7 * 11 * 13 *19),11 * 3);
    }

    #[test]
    fn test_two_sum(){
        let target = 9;
        let nums_example_two: Vec<i32> = vec![2,7,11,15]; 
        let two_sum_map_result = two_sum_map(nums_example_two,target);
        let nums_example_three: Vec<i32> = vec![3,2,4];
        let result = two_sum_map(nums_example_three,6);
        assert_eq!(two_sum_map_result, vec![0,1]);
        assert_eq!(result, vec![1,2]);
    }