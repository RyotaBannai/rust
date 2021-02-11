use crossbeam;

pub fn test() {
  let my_arr = &[1, 25, -4, 10, 1, 25, -4, 10, 1, 25, -4, 10, 1, 25, -4, 100];
  let max_num = find_max_num(my_arr, 1);
  println!("{:?}", Some(max_num));
}

fn find_max_num(my_arr: &[i32], nth_call: i32) -> Option<&i32> {
  const THRESHOLD: usize = 2;
  if my_arr.len() <= THRESHOLD {
    return my_arr.into_iter().max();
  }

  let mid = my_arr.len() / 2;
  let (left, right) = my_arr.split_at(mid);

  crossbeam::scope(|s| {
    println!("{} spawning.", nth_call);
    let nth = nth_call + 1;
    let thread_l = s.spawn(move |_| find_max_num(left, nth));
    let thread_r = s.spawn(move |_| find_max_num(right, nth));

    let max_l = thread_l.join().unwrap()?;
    let max_r = thread_r.join().unwrap()?;

    Some(max_l.max(max_r))
  })
  .unwrap()
}
