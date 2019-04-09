use alloc::vec::Vec;
use arrayvec::ArrayVec;

/// Perform subtraction on unsigned values (absolute difference)
pub fn unsigned_subtraction(x: u16, y: u16) -> u16 {
    if x < y {
        y - x
    } else {
        x - y
    }
}

///Average over values in a vector
pub fn average_vector(values: Vec<u16>) -> u16 {
    let mut sum: u16 = 0;
    let mut count: u16 = 0;
    for x in values {
        sum += x;
        count += 1;
    }
    sum / count
}

///Converts a list of tuples into a tuple of lists
pub fn get_tuple_of_lists<T>(list_of_tuples: Vec<(T, T)>) -> (Vec<T>, Vec<T>) {
    let mut vector_0 = Vec::new();
    let mut vector_1 = Vec::new();
    for tuple in list_of_tuples {
        vector_0.push(tuple.0);
        vector_1.push(tuple.1);
    }
    (vector_0, vector_1)
}
