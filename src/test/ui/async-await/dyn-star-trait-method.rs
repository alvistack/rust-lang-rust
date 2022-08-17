// run-pass
#![feature(async_fn_in_traits)]

trait Foo {
    fn get(&self) -> usize;
}

impl Foo for usize {
    fn get(&self) -> usize {
        *self
    }
}

fn invoke_dyn_star(i: dyn* Foo) -> usize {
    i.get()
}

fn make_and_invoke_dyn_star(i: usize) -> usize {
    let dyn_i: dyn* Foo = i as dyn* Foo;
    invoke_dyn_star(dyn_i)
}

fn main() {
    println!("{}", make_and_invoke_dyn_star(42));
}
