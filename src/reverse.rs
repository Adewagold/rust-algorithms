pub fn reverse_vector(my_vector: &mut Vec<i64>) -> &mut Vec<i64> {
    let mut index = my_vector.len()-1;
    for i in 0..my_vector.len() {
        let current_element = my_vector.remove(index -i);
        my_vector.push(current_element);
    }
    my_vector
}