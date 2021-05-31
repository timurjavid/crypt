# sp21-cs242-project: Encrypted Notes App in Rust

The purpose of this project is to create an notes application backend in Rust that stores files in an encrypted format.

# Build instructions
Rust and Cargo both need to be installed on your operating system. See here for more information: https://www.rust-lang.org/tools/install.

After you have these installed, you can simply run `cargo build --bin notes`, and the binary will be output in `notes/target/debug`. You can also run it using `cargo run` in lieu of the binary.

# Motivation

This project is final project in CS242 Spring 2021.

While there are different apps for keeping track of notes and tasks, none of them provide all of the functionality that I would like in a single package. Moreover, notes are often stored by large companies like Google, and privacy is not maintained. The goal is to eventually provide perfect forward secrecy with all notes utilizing the Signal Protocol and allow notes to be transferred to other devices. A complete rewrite of the Signal Protocol is out of scope for this course, so this library will be single-device and will utilize simple file encryptions.

# Code Style

I will mostly follow the styles provided here: Rust Style Guide (https://doc.rust-lang.org/1.0.0/style/)

# Test Plan

I will be using Rust unit testing and manual tests to test the functionality of the application.

# Code Information

The scope of this project is to provide a back-end that can be extended later on. As such, there will be no front-end component to this project, other than a simple command-line interface for testing.

## Rust
Learn more about Rust Lang here: https://www.rust-lang.org/
