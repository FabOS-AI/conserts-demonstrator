<!--
SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE

SPDX-License-Identifier: MIT
-->

# conserts demonstrator

FabOS AP3.2 Demonstrator

## How to build and run one of the systems

```sh
cd scanner
conserts compile --ros -i ./scanner/scanner.yml
# wsl if on Windows and entering the WSL
export ROSRUST_MSG_PATH=$(pwd)/target
cd app
cargo run --release
```

## License

Licensed under MIT license.
