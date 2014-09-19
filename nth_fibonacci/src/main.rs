fn fibonacci(n: uint) -> uint {
  fn _fibonacci(n: uint, a:uint, b:uint) -> uint {
    match (n, a, b) {
      (0, _, _) => a,
      _         => _fibonacci(n - 1, b, a + b)
    }
  }
  _fibonacci(n, 0, 1)
}

#[test]
fn fifth_fibonacci_number() {
  let result = fibonacci(5);
  assert_eq!(result, 5);
}

#[test]
fn thirtieth_fibonacci_number() {
  let result = fibonacci(30);
  assert_eq!(result, 832040);
}
