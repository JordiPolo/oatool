# Google discovery spec


Rust crate for serializing and deserializing from the Google discovery specification format (yaml version).


## Installing

Add the following to your `Cargo.toml` file

```toml
[dependencies]
google_discover_spec = "0.1"
```



## Using

```rust
extern crate google_discovery_spec;

fn main() {
  match google_discovery_spec::from_path("path/to/contract.yaml") {
    Ok(spec) => println!("spec: {:?}", spec),
    Err(err) => println!("error: {}", err)
  }
}
```


