#[macro_export]
macro_rules! to_string {
   ($input:expr) => {
      String::from($input)
   }
}

#[macro_export]
macro_rules! to_int {
   ($input:expr) => {
      $input.parse::<i32>().unwrap();
   }
}
