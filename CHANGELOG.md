# Change Log

## 0.1.2 (2018-02-03)
### Added
* Added a simple kernel build in the root workspace
  * Kernel sets and clears GPIO 16 repeatedly, to simulate a blinking light
  * Added necessary firmware in the firmware/ directory
  * Created a Makefile for a simpler build process
* Added a rustfmt.toml file to set max line width to 80

## 0.1.1 (2018-01-29)
### Added
* Added a change log to keep track of changes made on releases.

### Changed
* Added various fields to root Cargo.toml.

### Fixed
* Added year and name to license file.
