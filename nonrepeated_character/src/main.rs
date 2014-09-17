use std::io;
use std::collections::hashmap::HashMap;

fn main() {
  println!("Input a string: ");
  let input = io::stdin()
                .read_line()
                .ok()
                .expect("Failed to read input.");
  println!("First non-repeated character: {}", first_nonrepeated_character(input.as_slice()));
}

fn first_nonrepeated_character(input_str: &str) -> char {
  let mut char_hash:HashMap<char, int> = HashMap::new();

  for c in input_str.chars() {
    let is_none = match char_hash.find_mut(&c) {
      None => true,
      Some(val) => { *val += 1; false }
    };

    if is_none {
      char_hash.insert(c, 1i);
    }
  }

  let answer: char = '-';

  for (k, v) in char_hash.iter() {
    if *v == 1 {
      let answer = k;
      return *answer;
    }
  }
  answer
}

/*#[test]
fn first_character_non_repeated() {
  assert!(first_nonrepeated_character("ABBC") == "A");
}

#[test]
fn middle_character_non_repeated() {
  assert!(first_nonrepeated_character("AABCCD") == "B");
}

#[test]
fn no_repeated_characters() {
  assert!(first_nonrepeated_character("ABCD") == "-");
}

#[test]
fn non_string() {
  assert!(first_nonrepeated_character(ABCD) == "-");
}
*/
