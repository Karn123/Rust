use std::collections::HashSet;
use std::hash::Hash;

fn main() {
  println!("The first array is a subset of the second array: {}", is_subset_of(["commit", "push"], ["commit", "push", "rebase", "push", "blame"]));
}

fn is_subset_of<T: Eq + Hash>(first_arr: &[T], second_arr: &[T]) -> bool {
  let mut set: HashSet<&T> = HashSet::new();
  for i in second_arr.iter() {
    set.insert(i);
  }
  for i in first_arr.iter() {
    if !set.contains(&i) {
      return false;
    }
  }
  true
}

#[test]
fn test_one() {
  let result = is_subset_of(["commit", "push"], ["rebase", "blame", "origin", "master"]);
  assert_eq!(result, false);
}

#[test]
fn test_two() {
  let result = is_subset_of(["commit", "push"], ["rebase", "blame", "origin", "master", "push", "commit"]);
  assert_eq!(result, true);
}

#[test]
fn test_three() {
  let result = is_subset_of(["commit", "push"], ["commit"]);
  assert_eq!(result, false);
}

#[test]
fn test_four() {
  let result = is_subset_of([1u, 2, 3], [3u, 4, 1, 2]);
  assert_eq!(result, true);
}
