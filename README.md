# json_macro

Macro for rust_serialize crate realized with standard macro_rule (no needs compiler features).

## Dependencies

Add this to your `Cargo.toml`:

```toml
[dependencies]
json_macro = "*"
```

and this to your crate root:

```rust
#[macro_use]
extern crate json_macro;
```

## Usage

Use macro this way:

```rust
let s = json!("string");
let f = json!(3.1f64);
let a = json!([1,2,3,4,5]);
let o = json!({
	"one" => 1,
	"sub" => (json!({
		"two" => 2
	}))
});
```
