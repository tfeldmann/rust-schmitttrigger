# Rust Schmitt-Trigger library

Basic usage:

```rust
let mut trigger = SchmittTrigger::new(5, 10);
trigger.input(4)  // false
trigger.input(12) // true
trigger.input(6)  // still true
let result = trigger.output() // true
```

Works with any type that has the PartialEq trait.
