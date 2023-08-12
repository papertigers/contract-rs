# libcontract

Safe wrapper around [libcontract(3lib)](https://illumos.org/man/3LIB/libcontract).

## Example

```rust
use libcontract::status::{ContractStatus, Detail};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let ctid = match args.iter().nth(1) {
        Some(cid) => cid.parse().unwrap(),
        None => {
            eprintln!("Usage: {} cid", args.first().unwrap());
            std::process::exit(1);
        }
    };

    let status = ContractStatus::new(ctid, Detail::All).unwrap();
    println!(
        "members in contract {ctid}:\n {:#?}",
        status.get_members().unwrap()
    );
}
```
