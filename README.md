
# Kmbox Crate

A QOL crate for connecting to kmbox devices


## Features

- Auto find kmbox b+ pro
- Simple command to send inputs
## Roadmap

- Add Support for Kmbox Net

- Add Support for Kmbox NVideo


## Usage/Examples

```rust
use kmbox::{find_port, move_command};
use colored::*;

fn main() {
    match find_port() {
        Some(port_name) => {
            if let Err(e) = move_command(port_name.clone(), 100, 200) {
                eprintln!("{}", e.red());
            }

            if let Err(e) = move_command(port_name, 300, 400) {
                eprintln!("{}", e.red());
            }
        }
        None => eprintln!("{}", "KMBox not found".red()),
    }
}
```


## Contributing

Contributions are always welcome!

Please fork and make a PR with Contributions and ill add if its worth adding.


## Documentation

[Documentation](https://docs.rs/kmbox/latest/kmbox/)

[Crates.io](https://crates.io/crates/kmbox)


## FAQ

#### Why is kmbox not found?

It can be something simple of the com port just not being shown or drivers not being installed

#### When will kmbox Net and NVideo support be added?

Im crippling poor so not for awhile unless someone wants to donate me money via crypto to add support i will

