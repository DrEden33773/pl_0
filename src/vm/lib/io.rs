use crate::{value::Value, vm::advanced::VMFrame};

pub(super) fn lib_write(state: &mut VMFrame) -> i32 {
  for i in 1..=state.get_top() {
    if i != 1 {
      print!("\t");
    }
    print!("{}", state.get::<&Value>(i));
  }
  println!();
  0
}

pub(super) fn lib_read(state: &mut VMFrame) -> i32 {
  let mut input = String::new();
  std::io::stdin().read_line(&mut input).unwrap();
  let input: Vec<i64> = input
    .split_whitespace()
    .map(|x| x.parse::<i64>().unwrap())
    .collect();
  for i in 1..=state.get_top() {
    state.set(i, Value::Integer(input[i - 1]));
  }
  0
}
