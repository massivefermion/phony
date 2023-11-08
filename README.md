![phony](https://raw.githubusercontent.com/massivefermion/phony/main/banner.png)

[![Package Version](https://img.shields.io/hexpm/v/phony)](https://hex.pm/packages/phony)
[![Hex Docs](https://img.shields.io/badge/hex-docs-ffaff3)](https://hexdocs.pm/phony/)

# phony

An international phone number validator

## <img width=32 src="https://raw.githubusercontent.com/massivefermion/phony/main/icon.png"> Quick start

```sh
gleam test  # Run the tests
gleam shell # Run an Erlang shell
```

## <img width=32 src="https://raw.githubusercontent.com/massivefermion/phony/main/icon.png"> Installation

This package can be added to your Gleam project:

```sh
gleam add phony
```

and its documentation can be found at <https://hexdocs.pm/phony>.

## <img width=32 src="https://raw.githubusercontent.com/massivefermion/phony/main/icon.png"> Usage

```gleam
import phony

pub fn main() {
    phony.validate("+1 4305573966")
    phony.validate("+49 1522 343333")
    phony.validate_by_country("09101752439", "IR")
    phony.validate_by_code("27111234", "880")
}
```