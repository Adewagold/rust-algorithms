
mod find;
mod reverse;

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
}
