pub fn owner_ship() {
  let s1 = String::from("hello");
  let ref_s1 = &s1;
  ref_s1.push_str(", xxx");

  print!("{}, {}", s1, ref_s1.len());
}