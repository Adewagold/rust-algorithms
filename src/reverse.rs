use core::num;

pub fn reverse_vector(my_vector: &mut Vec<i64>) -> &mut Vec<i64> {
    let mut index = my_vector.len()-1;
    for i in 0..my_vector.len() {
        let current_element = my_vector.remove(index -i);
        my_vector.push(current_element);
    }
    my_vector
}

/**
 * For instance rotate a vector by one
 * Input:  0, 1, 2, 3, 4, 5, 6
 * Output: 6, 0, 1, 2, 3, 4, 5
 * Pass the value of vector by mutable reference to prevent ownership of moving input vector to variable my_vector
 * return my_vector reference
 */
pub fn rotate_vector_once(my_vector: &mut Vec<i64>) -> &mut Vec<i64> {
    let last_digit = my_vector[my_vector.len()-1];
    let mut current_number:i64 = my_vector[0];
    let mut current_temp = my_vector[0];
    for i in 1..my_vector.len() {
        current_number = my_vector[i];
        my_vector[i] = current_temp;
        current_temp = current_number;
    }
    my_vector[0]=last_digit;
    my_vector
}


/**
 * For instance rotate a vector by n number of times clock-wise
 * Input:  0, 1, 2, 3, 4, 5, 6
 * Output: 6, 0, 1, 2, 3, 4, 5
 * Pass the value of vector by mutable reference to prevent ownership of moving input vector to variable my_vector
 * return my_vector reference
 */
pub fn rotate_vector_multiple_times(my_vector: &mut Vec<i64>, number_of_times: i32) -> &mut Vec<i64>{
    for i in 0..number_of_times{
        rotate_vector_once(my_vector);
    }
    my_vector
}

#[test]
pub fn test_rotate_vector_once(){
    let mut sample_vector = vec![2,7,11,15]; 
    let result = rotate_vector_once(&mut sample_vector).to_vec();
    let expected = vec![15, 2, 7, 11];
    assert_eq!(expected,result);

}


#[test]
pub fn test_rotate_vector_multiple_times(){
    let mut sample_vector_ii = vec![0,1,2,3,4,5,6,7,8];
    let expected = vec![6,7,8,0,1,2,3,4,5];
    let sample_vector_two_relax = rotate_vector_multiple_times(&mut sample_vector_ii, 3).to_vec();
    assert_eq!(expected, sample_vector_two_relax);
}