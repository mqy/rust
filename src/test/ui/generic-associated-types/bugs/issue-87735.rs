// check-fail

// This should pass, but we need an extension of implied bounds (probably).

#![feature(generic_associated_types)]

pub trait AsRef2 {
  type Output<'a> where Self: 'a;

  fn as_ref2<'a>(&'a self) -> Self::Output<'a>;
}

impl<T> AsRef2 for Vec<T> {
  type Output<'a> where Self: 'a = &'a [T];

  fn as_ref2<'a>(&'a self) -> Self::Output<'a> {
    &self[..]
  }
}

#[derive(Debug)]
struct Foo<T>(T);
#[derive(Debug)]
struct FooRef<'a, U>(&'a [U]);

impl<'b, T, U> AsRef2 for Foo<T> //~ the type parameter
where
    // * `for<'b, 'c> T: AsRef2<Output<'b> = &'c [U]>>` does not work
    //
    // * `U` is unconstrained but should be allowed in this context because `Output` is
    // an associated type
    T: AsRef2<Output<'b> = &'b [U]>,
    U: 'b
{
  type Output<'a> where Self: 'a = FooRef<'a, U>;

  fn as_ref2<'a>(&'a self) -> Self::Output<'a> {
    FooRef(self.0.as_ref2())
  }
}

fn main() {
    let foo = Foo(vec![1, 2, 3]);
    dbg!(foo.as_ref2());
}
