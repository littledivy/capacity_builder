use string_capacity::StringBuilder;

fn main() {
  // Run registered benchmarks.
  divan::main();
}

#[divan::bench]
fn string_builder() -> usize {
  StringBuilder::build(|builder| {
    for i in 0..1_000 {
      builder.append("Hello, ");
      builder.append("world!");
      builder.append(" Number: ");
      builder.append(i);
      builder.append(' ');
    }
  })
  .unwrap()
  .len()
}

#[divan::bench]
fn text_new() -> usize {
  let mut text = String::new();
  for i in 0..1_000 {
    text.push_str("Hello, ");
    text.push_str("world!");
    text.push_str(" Number: ");
    text.push_str(i.to_string().as_str());
    text.push(' ');
  }
  text.shrink_to_fit();
  text.len()
}
