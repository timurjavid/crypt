# Encrypted Notes App in Rust

The purpose of this project is to create an notes application in Rust that stores files in an encrypted format.

# Build instructions
Rust and Cargo both need to be installed on your operating system. See here for more information: https://www.rust-lang.org/tools/install.

After you have these installed, you can simply run `cargo build --bin notes`, and the binary will be output in `notes/target/debug`. You can also run it using `cargo run` in lieu of the binary.

# TODO

* Front end
   * The application is only a command-line interface. Removing this command-line interface and adding a front end will allow for a better user experience. 
* Rework the encryption and decryption functionality to allow for more efficient searches.
   * The application searches through every file in a "notes" folder and attempts to decrypt. A possible solution for this is to keep the notes decrypted while the application is running, then encrypt them when the application exits.
     * This has consequences on unexpected exits.
* Refactor searching on tasks/subtasks
   * The application searches through tasks and subtasks in a similar way. The code could be simplified since tasks and subtasks share the same structure.

# Motivation

This project is final project in CS242 Spring 2021.

While there are different apps for keeping track of notes and tasks, none of them provide all of the functionality that we would like in a single package. Moreover, notes are often stored by large companies like Google, and privacy is not maintained. The goal is to eventually provide perfect forward secrecy with all notes utilizing the Signal Protocol and allow notes to be transferred to other devices. 

# Code Style

The code style mostly follow the styles provided here: Rust Style Guide (https://doc.rust-lang.org/1.0.0/style/)

# Test Plan

The project uses Rust unit testing and manual tests to test the functionality of the application.

# Code Information

The scope of this project is to provide a back-end that can be extended later on. As such, there will be no front-end component to this project, other than a simple command-line interface for testing.

## Rust
Learn more about Rust Lang here: https://www.rust-lang.org/
