use crate::solution::*;
use std::env;
use std::str::FromStr;

mod find;
mod reverse;
mod solution;

fn main() {
    /*Find the max of two numbers A and B */
    let a = 3;
    let b = 5;
    let max_of_a_and_b = find::max(a, b);
    println!("The max of A and B is {}", max_of_a_and_b);

    /*Find the min of two numbers A and B */
    let min_of_a_and_b = find::min(a,b);
    println!("The max of A and B is {}", min_of_a_and_b);

    /* Reverse a vector */
    let mut my_vector: Vec<i64> = vec![0,1,2,3,4,5,6];
    let reversed_vector = reverse::reverse_vector(&mut my_vector);
    println!("{:?}", reversed_vector);

    /* Two Sum */
    let nums: Vec<i32> = vec![2,7,11,15]; 
    let target = 9;
    let twosum = two_sum(nums, target);
    println!("{:?}", twosum);
    let target = 9;
    let nums_example_two: Vec<i32> = vec![2,7,11,15]; 
    let nums_example_three: Vec<i32> = vec![3,2,4];
    let two_sum_map_result = two_sum_map(nums_example_two,target);
    let two_sum_map_result_ii = two_sum_map(nums_example_three,6);
    println!("{:?}", two_sum_map_result);
    println!("{:?}", two_sum_map_result_ii);

    /* Greatest common divisor of two integers */
    let m = 15;
    let n = 14;
    let common_divisor = greatest_common_divisor(n, m);
    println!("Common divisor of n: {} and m: {} is {}", n,m,common_divisor);

    /* */
    let mut numbers = Vec::new();
    for arg in env::args().skip(1){
        numbers.push(u64::from_str(&arg).expect("Error parsing argument"));
    }

    if numbers.len()  < 1 {
        eprintln!("Usage: GCD number ...");
        std::process::exit(1);
    }


    let mut d = numbers[0];
    for m in &numbers[1..]{
        d = greatest_common_divisor(d, *m);
        println!("Iteration {}, result {}", m,d);
    }
    println!("The greatest common divisor of {:?} is {}.", numbers,d);
}
