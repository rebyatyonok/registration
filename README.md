# Registration-lib
This project a little bit messy now. I plan to split it in a two separate: a library and a server implementation. Maybe later add a front-end using any interesting framework.

This is a simple rust library based on one of my working projects. That lib should be able to allow users to register for some classes on a particular date. If there is more than 6 users on a  date - it should be closed for the registration. 

## What’s the point?
I write it only for Rust practice.

## Installation
You must have a [Diesel](http://diesel.rs) installed. If you have, run:
```
diesel migration run
```
This will crate a SQLite database and apply all migrations needed for initial setup. Also, it will create a `src/schema.rs` file, that you can delete, because it is already placed in `src/db/` folder. 

Then run:
```
cargo build
```
That’s all.

## Running tests
Before running all tests, you should set you database to initial state. If it is, run `cargo test`, if it’s not - undo all changes in database with `diesel migration redo`, and then `diesel migration run` again. Then you can run `cargo test`.

This is not good example of unit and integrational tests, because it require a particular database state to run. **If you have a nice example of how to do it in a way better, please let me know.**
