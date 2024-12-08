use string_capacity::StringBuilder;

fn main() {
  // Run registered benchmarks.
  divan::main();
}

#[divan::bench]
fn string_builder() -> usize {
  StringBuilder::build(|builder| {
    builder.append("Hello, ");
    builder.append("world!");
    builder.append("testing ");
  })
  .unwrap()
  .len()
}

#[divan::bench]
fn text_new() -> usize {
  let mut text = String::new();
  text.push_str("Hello, ");
  text.push_str("world!");
  text.push_str(" testing ");
  text.shrink_to_fit();
  text.len()
}
