trait Foo {
  fn answer(self);
}

struct NoData<T>;
//~^ ERROR: parameter `T` is never used

impl<T> Foo for T where NoData<T>: Foo {
  //~^ ERROR: overflow evaluating the requirement
  //~| ERROR: overflow evaluating the requirement
  fn answer(self) {
    let val: NoData<T> = NoData;
  }
}

fn main() {}
