//! This is an unusual feature gate test, as it doesn't test the feature
//! gate, but the fact that not adding the feature gate will cause the
//! diagnostic to not emit the custom diagnostic message
//!
#[diagnostic::on_move(
//~^ ERROR the `#[diagnostic::on_move]` attribute is an experimental feature [E0658]
    message = "Foo"
)]
#[derive(Debug)]
struct Foo;

fn takes_foo(_: Foo) {}

fn main() {
    let foo = Foo;
    takes_foo(foo);
    let bar = foo;
    //~^ERROR use of moved value: `foo`
}
