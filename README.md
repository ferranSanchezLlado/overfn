# Overload functions (overfn)

This crate allows you to overload functions with the same name but with different number of arguments through the
[`overload`](https://docs.rs/overfn/latest/overfn/attr.overload.html) macro. After overloading all the functions, you 
need to use the [`macros!()`](https://docs.rs/overfn/latest/overfn/macro.macros.html) to genarate the macros to invoke 
the overloaded functions.

## Example

```rust
use overfn::*;

#[overload]
fn test(item: usize) -> usize {
    item
}

#[overload]
fn test(left: usize, right: usize) -> usize {
    left + right
}

struct Test(usize);

impl Test {
    #[overload(Test)]
    fn new() -> Self {
        Self(0)
    }

    #[overload(Test)]
    fn new(item: usize) -> Self {
        Self(item)
    }

    #[overload(Test)]
    fn test(&self) -> usize {
        self.0
    }

    #[overload(Test)]
    fn test(&self, other: usize) -> usize {
        self.0 + other
    }
}

macros!();

assert_eq!(test!(2), 2);
assert_eq!(test!(2, 2), 4);

let test = Test_new!();
assert_eq!(test.0, 0);

let test = Test_new!(2);
assert_eq!(test.0, 2);

assert_eq!(Test_test!(test), 2);
assert_eq!(Test_test!(test, 2), 4);
```

## Documentation

You can find the documentation [here](https://docs.rs/overfn).

## Limitations

- Curretly, you can't overload a function with the same number of arguments with different types.
- You need to use the [`macros!()`](https://docs.rs/overfn/latest/overfn/macro.macros.html) macro to generate the macros
  to call the overloaded functions.
- If you overload a class method or instance method, you need to pass the class name in the attribute.

## License

This project is licensed under the [MIT license](LICENSE-MIT) or [Apache License, Version 2.0](LICENSE-APACHE) at your option.
