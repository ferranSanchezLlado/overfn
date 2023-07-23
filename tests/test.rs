use overfn::*;

#[overload]
fn add(item: usize) -> usize {
    10 + item
}

#[overload]
fn add(left: usize, right: usize) -> usize {
    left + right
}

#[overload]
fn add(left: usize, right: usize, other: usize) -> usize {
    left + right + other
}

macros!();

#[test]
fn it_works() {
    let result = add!(2);
    assert_eq!(result, 12);

    let result = add!(2, 2);
    assert_eq!(result, 4);

    let result = add!(2, 2, 2);
    assert_eq!(result, 6);
}

struct Test {
    a: usize,
    b: usize,
}

impl Test {
    #[overload(Test)]
    fn new(a: usize) -> Self {
        Self { a, b: 0 }
    }

    #[overload(Test)]
    fn new(a: usize, b: usize) -> Self {
        Self { a, b }
    }

    #[overload(Test)]
    fn test(&self) -> usize {
        self.a + self.b
    }

    #[overload(Test)]
    fn test(&self, other: usize) -> usize {
        self.a + self.b + other
    }
}

macros!();

#[test]
fn test_class_method() {
    let result = Test_new!(2);
    assert_eq!(result.a, 2);
    assert_eq!(result.b, 0);

    let result = Test_new!(2, 2);
    assert_eq!(result.a, 2);
    assert_eq!(result.b, 2);
}

#[test]
fn test_object_method() {
    let test = Test_new!(2, 2);
    let result = Test_test!(test);
    assert_eq!(result, 4);

    let result = Test_test!(test, 2);
    assert_eq!(result, 6);
}
