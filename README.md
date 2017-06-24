# GPXConv

Fetching GPX data from a USB GPS tracker, adjusting elevation data with SRTM1 and creating a compressed Matlab binary file.

## Installation

In order to compile, install [*rustup*](https://www.rustup.rs/), install an appropriate Rust toolchain and run ```cargo build --release```

## Usage

The tool checks whether *gpsbabel.exe* is in the current directory. This executable file can be optained from the [GPSBabel](https://www.gpsbabel.org/) website. If the executable is found, GPX data is being fetched from a USB GPS tracker and converted. Otherwise, all GPX files in the current directory will be converted.
