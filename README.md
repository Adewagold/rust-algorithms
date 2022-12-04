# Rust Algorithms
This is a personal repository to commit different algorithms I write using Rust programming language as I continue to learn and contribute to other projects.

## Reverse a vector
```
pub fn reverse_vector(my_vector: &mut Vec<i64>) -> &mut Vec<i64> {
    let mut index = my_vector.len()-1;
    for i in 0..my_vector.len() {
        let current_element = my_vector.remove(index -i);
        my_vector.push(current_element);
    }
    my_vector
}
```

## Find max and min of A and B
```
mod find;

fn main() {
    /*Find the max of two numbers A and B */
    let a = 3;
    let b = 5;
    let max_of_a_and_b = find::max(a, b);
    println!("The max of A and B is {}", max_of_a_and_b);

    /*Find the min of two numbers A and B */
    let min_of_a_and_b = find::min(a,b);
    println!("The max of A and B is {}", min_of_a_and_b);
}
```
# Two Sum
Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

You can return the answer in any order.

## Example 1: (Leetcode link)[https://leetcode.com/problems/two-sum/description/]
```
Input: nums = [2,7,11,15], target = 9
Output: [0,1]
Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
```
