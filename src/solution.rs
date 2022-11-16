    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();;
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