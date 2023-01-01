use crate::cmdargs::{parse_complex, parse_pair, render, write_image, pixel_to_point};
use crate::solution::*;
use std::env;
use std::str::FromStr;

mod find;
mod reverse;
mod solution;
mod linkedlist;
mod cmdargs;
mod guess_game;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() !=5 {
        eprintln!("Usage: {} FILE PIXELS UPPERLEFT LOWERRIGHT", args[0]);
        eprintln!("Example: {} mandel.png 1000x750 -1.20,0.35 -1.0,0.20", args[0]);
        std::process::exit(1);
    }

    let bounds:(usize, usize) = parse_pair(&args[2], 'x').expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[3]).expect("error parsing upper left corner point");
    let lower_right = parse_complex(&args[4]).expect("error parsing lower right corner point");
    
    let mut pixels = vec![0; bounds.0 * bounds.1];
    // Make it multithreaded
    let threads  = 8;
    let rows_per_band = bounds.1 / threads + 1;
    {
        let bands: Vec<&mut[u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();
        crossbeam::scope(|spawner|{
            for (i,band) in bands.into_iter().enumerate(){
                let top = rows_per_band * i;
                let height = band.len() / bounds.0;
                let band_bounds = (bounds.0, height);
                let band_upper_left = pixel_to_point(bounds, (0,top), upper_left, lower_right);
                let band_lower_right = pixel_to_point(bounds, (bounds.0, top+height), upper_left, lower_right);
                spawner.spawn(move |_|{
                    render(band, band_bounds, band_upper_left, band_lower_right);
                });
            }
        }).unwrap();
    }
    render(&mut pixels, bounds, upper_left, lower_right);
    write_image(&args[1], &pixels, bounds).expect("error writing PNG file");

    guess_game::guess();

    let mut ll = linkedlist::LinkedList::new();
    ll.push_front(2);
    ll.push_back(12);
    ll.push_front(1);
    println!("ll = {:?}", ll);
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


    println!("A simple function to rotate the items of a vector clock-wise");
    let mut sample_vector = vec![2,7,11,15]; 
    let result = reverse::rotate_vector_once(&mut sample_vector).to_vec();
    println!("The rotated result of vector{:?} is {:?}", sample_vector,result);


    let number_of_times = 4;
    println!("A simple function to rotate the items of a vector clock-wise {} times",number_of_times);
    let mut sample_vector_ii = vec![0,1,2,3,4,5,6,7,8];
    let sample_vector_two_relax = reverse::rotate_vector_multiple_times(&mut sample_vector_ii, number_of_times).to_vec();
    println!("The rotated result of vector{:?} is {:?}", sample_vector,sample_vector_two_relax);


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
