use crate::{value::Value, vm::advanced::VMFrame};

pub(crate) fn lib_write(state: &mut VMFrame) -> i32 {
  for i in 1..=state.get_top() {
    if i != 1 {
      print!("  ");
    }
    print!("{}", state.get::<&Value>(i));
  }
  println!();
  0
}

pub(crate) fn lib_read(state: &mut VMFrame) -> i32 {
  let mut input = String::new();
  std::io::stdin().read_line(&mut input).unwrap();
  let input: Vec<i64> = input
    .split_whitespace()
    .map(|x| x.parse::<i64>().unwrap())
    .collect();
  for i in 1..=state.get_top() {
    state.set::<Value>(i, input[i - 1].into());
  }
  0
}
