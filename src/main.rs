
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
