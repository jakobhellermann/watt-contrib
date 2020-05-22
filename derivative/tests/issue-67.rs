#[cfg(feature = "use_core")]
extern crate core;

#[macro_use]
extern crate derivative;

#[derive(Derivative)]
#[derivative(Hash)]
enum Enumeration<T> {
    _Variant(T),
}
