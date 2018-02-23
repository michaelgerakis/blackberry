# Change Log

## Unreleased
### Changed
* Use compiler-builtins mem feature in favor of custom mem operations in lang_items.
* Use new firmware recieved from the official raspberry firmware [repository](https://github.com/raspberrypi/firmware).
  * Addition of fixup.dat correctly uses all 1GB of RAM

### Added
* Interface for atags structure located at 0x100.

## 0.2.0 (2018-02-19)
### Added
* Created a bootloader binary that listens for XMODEM communication, reads the given binary, and executes it.

### Changed
* Added xmodem abilities to ttywrite
* ```make install``` - Now runs ttywrite and sends the binary to the tty device

### Fixed
* xmodem library did not seem to work correctly, used reference code to fix it.
* read timeout in uart was set to nanoseconds instead of milliseconds

## 0.1.4 (2018-02-18)
### Added
* Added interface for UART pins on the raspberry
* Added a global static console object that mimics a console by writing to the UART
* Implemented a shell on top of the console singleton

## 0.1.3 (2018-02-10)
### Added
* Volatile library based off of a similar library by Sergio Benitez
  * Provides interfaces around raw pointers for performing volatile operations
* Added gpio and timer modules in the kernel for interfacing with the gpio and timer peripherals
* Added a .rustfmt.toml file to configure the formatting

### Changed
* Changed what happens in the main kernel loop. GPIO pins 5 and 19 blink repeatedly.
* Added various rules to the Makefile to make tasks easier
  * ```make install``` - Installs the binary in the usb and unmounts it
  * ```make check``` - Checks the rust code without compiling it
  * ```make format``` - Formats all rust dependencies (currently **_src/*.rs_** and **_volatile/src/*.rs_**)
  * ```make deps``` - Install all dependencies necessary for the project

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
