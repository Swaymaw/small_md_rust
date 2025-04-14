- In rust everything is an expressions and expression evaluate to a value. However, if a line ends in a semi-colon it is a statement and statements. It is expected in the rust community to use return keyword only for early-returns and for the last statemment the value/expression block without the semi-colon is enough.

- String slices borrows, whatever is at an address from a different owner. String slices are pointers to a borrowed reference. String slices are always immutable. We say it is not the owner because whenever a string slice is declared during compile time it lives in the binary code of the file itself and cannot be changed.

- Rust provides key from the manifest file (Cargo.toml) during compilation as environment variables.

- Rust automatically closes reader/file after it goes out of scope.
