# Ulam Spiral drawer
Ulam Spiral drawer is responsible for rendering [Ulam spiral](https://en.wikipedia.org/wiki/Ulam_spiral) fo file.
Ulam spiral displays an interesting property of prime numbers: they appear in relatively diagonal positions.
The length of diagonal of successive prime number may vary.

## Building
Ulam Spiral drawer can be easily build running simple build command:
```
cargo build --release
```

## Program arguments
Ulam spiral takes two arguments:
* --matrix_dimension (-m) - number of elements per row/column. Currently only odd numbers are supported. 
* --output_path (-o) - path to file to which image will be rendered.

## Example
```
cargo run --release -- -m 101 -o my_awesome_spiral.png
```