# Testing Struct and Union implementation in RUST

![Language](https://img.shields.io/badge/Language-RUST-critical?style=for-the-badge&logo=rust)
![System](https://img.shields.io/badge/System-Windows-A100FF?style=for-the-badge&logo=windows)

## Disclaim : Unsafe Code

This is a unsafe code, made to test ["C like"](https://github.com/mateusns12/CPP-Union-RXTX-Packet-Structure) union implementation, used to send struct data by acessing its byte value inside a union.

The basic purpose is to learn and find how to acess struct fields inside of a unstable union. No guide line followed, just making the code work by fixing compiler errors and warnings.

## Notes

- Im aware that values should be acessed by methods.
- Im aware of the enum type, and union limitations.
- Set method implemented, working.
- Using rust 1.60.0 
- Using nightly-x86_64-pc-windows-msvc as toolchain.
- Compiler warnings disabled

## To-do

 - [ ] Implement Get methods.
 - [ ] Change union declaration
 - [ ] Test shared acess between threads with Arc

