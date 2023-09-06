# Place Capitals Crate

The **Place Capitals Crate** is a Rust library designed to provide easy access to static data related to countries, their capitals, and USA states and their capitals. This crate offers convenient functions to check whether a given place is a country or a USA state, as well as the ability to retrieve the capital for a given place.

## Features

- **Place Type Detection**: Determine whether a given place is a country or a USA state.
- **Capital Lookup**: Retrieve the capital associated with a specific country or USA state.

## Installation

To use the Place Capitals Crate in your Rust project, add it as a dependency in your `Cargo.toml` file:

```toml
[dependencies]
place-capitals = "0.1.0"
```

## Usage

First, import the crate in your Rust project:

```rust
extern crate place_capitals;
use place_capitals::PlaceDetector;
```

### Example 1: Detecting Place Type

```rust
fn main() {
    // Create a new PlaceDetector instance
    let detector = PlaceDetector::default();
    
    // Check if a place is a country
    let is_country = detector.is_country("France");
    println!("Is it a country? {}", is_country); // Output: Is it a country? true
    
    // Check if a place is a USA state
    let is_usa_state = detector.is_usa_state("California");
    println!("Is it a USA state? {}", is_usa_state); // Output: Is it a USA state? true
    
    // Check an unknown place
    let is_unknown = detector.is_country("Mars");
    println!("Is it a country? {}", is_unknown); // Output: Is it a country? false
}
```

### Example 2: Retrieving Capital

```rust
fn main() {
    // Create a new PlaceDetector instance
    let detector = PlaceDetector::default();
    
    // Get the capital of a country
    let country_capital = detector.get_capital("France");
    println!("Capital of France: {:?}", country_capital); // Output: Capital of France: Some("Paris")
    
    // Get the capital of a USA state
    let usa_state_capital = detector.get_capital("California");
    println!("Capital of California: {:?}", usa_state_capital); // Output: Capital of California: Some("Sacramento")
    
    // Get the capital of an unknown place
    let unknown_capital = detector.get_capital("Mars");
    println!("Capital of Mars: {:?}", unknown_capital); // Output: Capital of Mars: None
}
```

## License

This crate is distributed under the terms of the MIT License. See the [LICENSE](LICENSE) file for details.

## Contribution

Contributions to this crate are welcome! If you encounter any issues or have ideas for improvements, please open an issue or submit a pull request on the [GitHub repository](https://github.com/GeethaPardheev/place_capitals).