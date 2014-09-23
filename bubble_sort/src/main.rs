fn bubble_sort<T: std::cmp::Ord>(array: &mut [T]) {
    let mut done = false;
    while !done {
        done = true;
        for mut i in range(1, array.len()) {
            if array[i-1] > array[i] {
                done = false;
                array.swap(i, i-1);
                i = i-1;
            }
        }
    }    
}

fn main() {
    let mut array = [31i, 41, 59, 26, 41, 58, 60];
    bubble_sort(array);
    for j in range(0u, array.len()) {
        println!("{}", array[j]);
    }
}

#[test]
fn main_test() {
    let result = bubble_sort([31i, 41, 59, 26, 41, 58, 60]);
    assert_eq!(result, [26i, 31, 41, 41, 58, 59, 60]);
}
