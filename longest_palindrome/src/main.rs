fn make_vector_from_string(input: &'static str) -> Vec<char> {
  let mut vec: Vec<char> = Vec::<char>::new();
  for c in input.chars() {
    vec.push(c);
  }
}

fn is_palindrome(input: &Vec<char>, i: int, j: int) -> Option<String> {
  let length: uint = input.len();
  let mut left = i;
  let mut right = j;
  while left >= 0 && right < length as int && input[left as uint] == input[right as uint] {
    left -= 1;
    right += 1;
  }
  let substring = input.slice(left as uint, right as uint);
  let s = String::from_chars(substring);
}

fn longest_palindrome(input: &'static str) -> String {
  let mut result: String = String::new();
  let chars = make_vector_from_string(input);

  for i in range(1, chars.len()) {
    let odd = is_palindrome(&chars, (i - 1) as int, (i + 1) as int);
    if odd.len() > result.len() {
      result = odd;
    }
    let even = is_palindrome(&chars, i as int, (i + 1) as int);
    if even.len() > result.len() {
      result = even;
    }
  }
  result
}

fn main() {
  println!("Longest palindrome: {}", longest_palindrome("My dad is a racecar athlete"));
}
