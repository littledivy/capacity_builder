# `capacity_builder`

Builders where the code to calculate the capacity is the same as the code to
write what's being built.

## Overview

Sometimes you have some complex code that would be a bit of a pain to calculate
the capacity of or could risk easily getting out of sync with the
implementation. This crate makes keeping it in sync easier because it's the same
code.

```rs
use capacity_builder::StringBuilder;

let text = StringBuilder::build(|builder| {
  for (i, import_module) in import_modules.iter().enumerate() {
    builder.append("// ");
    builder.append(i);
    builder.append(" import\n");
    builder.append("import \"");
    builder.append(import_module);
    builder.append("\";\n");
  }
})?;
```

Behind the scenes it runs the closure once to compute the capacity and a second
time to write the string.

## Features

1. The builder prevents adding owned data—only references.
   - This helps to prevent accidentally allocating data multiple times in the
     closure.
1. Errors when capacity cannot be reserved.
1. For the string builder, types other than references can be provided.
   - Numbers get written with the [itoa](https://crates.io/crates/itoa) crate.

## Tips

- Do any necessary allocations before running the closure.
- Measure before and after using this crate to ensure you're not slower.
