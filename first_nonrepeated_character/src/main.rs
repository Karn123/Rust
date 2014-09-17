use std::collections::hashmap::HashMap;

#[cfg(not(test))]
fn main() {
  let input = first_nonrepeated_character("ABA");
  match input {
    Some(val) => { assert_eq!("B", val)},
    None => assert!(false)
  }
}


#[cfg(not(test))]
// our function returns an option, which encapsulates the fact
// that it may return a valid char or 'None' otherwise
fn first_nonrepeated_character(input: &str) -> Option<char> {
  // instantiate a hash map to store key, value pairs
  let mut char_hash: HashMap<char, uint> = HashMap::new();
  // for each character in the input string
  for character in input.chars() {
    // either insert a character that isn't already in the hash map
    // or update the value associated with each key, which keeps a 
    // count of how many times each char has appeared so far
    char_hash.insert_or_update_with(character, 1, |_key, val| *val += 1 );
  }
  // loop through each character in input string again
  for character in input.chars() {
    // this time we check each character in the string against its running count value
    // if its count value equals 1
    if char_hash[character] == 1u {
      // it is our first non-repeated character
      return Some(character);
    }
  } 
  // otherwise None is returned
  None

}

#[test]
fn first_character_non_repeated() {
  let test: &str = "ABBCCC";
  let answer = first_nonrepeated_character(test);
  match answer {
    Some(val) => assert_eq!(val, "A"),
    None => assert!(false)
  }
}

#[test]
fn middle_character_non_repeated() {
  let test: &str = "AABCCD";
  let answer = first_nonrepeated_character(test);
  match answer {
    Some(val) => assert_eq!(val, "B"),
    None => assert!(false)
  }
}

#[test]
fn last_character_non_repeated() {
  let test: &str = "ACABACBD";
  let answer = first_nonrepeated_character(test);
  match answer {
    Some(val) => assert_eq!(val, "D"),
    None => assert!(false)
  }
}

#[test]
fn no_repeated_characters() {
  let test: &str = "ABCD";
  let answer = first_nonrepeated_character(test);
  match answer {
    Some(val) => assert!(false),
    None => assert!(true)
  }
}

