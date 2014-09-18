use std::collections::hashmap::HashMap;

fn main() {
  println!(is_subset_of(["commit", "push"], ["commit", "push", "rebase", "push", "blame"]));
}

fn is_subset_of(first_arr: [&str], second_arr: [&str]) -> bool {
  let mut hash: HashMap<&str, uint> = HashMap::new();
  for i in second_arr.iter() {
    hash.insert_or_update_with( *i, 1, |_key, val| *val += 1 );
  }
  for i in first_arr.iter() {
    if hash[*i] < 1u {
      return false;
    }
  }
  return true;
}
