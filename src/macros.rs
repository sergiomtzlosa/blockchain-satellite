
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

#[macro_export]
macro_rules! to_u16 {
   ($input:expr) => {
      $input.parse::<u16>().unwrap();
   }
}

#[macro_export]
macro_rules! try_operation {
    ($e:expr) => (match $e {
        Ok(val) => val,
        Err(err) => return Err(err)
    });
}
