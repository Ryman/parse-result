# Adds a `parse` method to `Result`

## Example

```rust,no_run
extern crate parse_result;
use parse_result::*;
use std::env;

fn main() {
    // It turns code like this:
    env::var("PORT").map(|s| s.parse().unwrap_or(3000)).unwrap_or(3000);

    // Into this:
    env::var("PORT").parse().unwrap_or(3000);

    // Matching to find the specific failure
    match env::var("PORT").parse::<u32>() {
        Ok(port) => println!("Parsed port {} successfully!", port),
        Err(OriginalErr(e)) => panic!("Failed to get PORT from env: {}", e),
        Err(ParseFailure(e)) => panic!("Failed to parse PORT: {}", e),
    }
}
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
