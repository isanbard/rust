// Copyright 2012-2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/*! Composable internal iterators

Internal iterators are functions implementing the protocol used by the `for` loop.

An internal iterator takes `fn(...) -> bool` as a parameter, with returning `false` used to signal
breaking out of iteration. The adaptors in the module work with any such iterator, not just ones
tied to specific traits. For example:

~~~~
use core::iter::iter_to_vec;
println(iter_to_vec(|f| uint::range(0, 20, f)).to_str());
~~~~

An external iterator object implementing the interface in the `iterator` module can be used as an
internal iterator by calling the `advance` method. For example:

~~~~
use core::iterator::*;

let xs = [0u, 1, 2, 3, 4, 5];
let ys = [30, 40, 50, 60];
let mut it = xs.iter().chain(ys.iter());
for it.advance |&x: &uint| {
    println(x.to_str());
}
~~~~

Internal iterators provide a subset of the functionality of an external iterator. It's not possible
to interleave them to implement algorithms like `zip`, `union` and `merge`. However, they're often
much easier to implement.

*/

pub trait Times {
    fn times(&self, it: &fn() -> bool);
}

/**
 * Transform an internal iterator into an owned vector.
 *
 * # Example:
 *
 * ~~~
 * let xs = ~[1, 2, 3];
 * let ys = do iter_to_vec |f| { xs.each(|x| f(*x)) };
 * assert_eq!(xs, ys);
 * ~~~
 */
#[inline(always)]
pub fn iter_to_vec<T>(iter: &fn(f: &fn(T) -> bool)) -> ~[T] {
    let mut v = ~[];
    for iter |x| { v.push(x) }
    v
}

/**
 * Return true if `predicate` is true for any values yielded by an internal iterator.
 *
 * Example:
 *
 * ~~~~
 * let xs = ~[1u, 2, 3, 4, 5];
 * assert!(any(|&x: &uint| x > 2, |f| xs.each(f)));
 * assert!(!any(|&x: &uint| x > 5, |f| xs.each(f)));
 * ~~~~
 */
#[inline(always)]
pub fn any<T>(predicate: &fn(T) -> bool, iter: &fn(f: &fn(T) -> bool)) -> bool {
    for iter |x| {
        if predicate(x) {
            return true
        }
    }
    false
}

/**
 * Return true if `predicate` is true for all values yielded by an internal iterator.
 *
 * # Example:
 *
 * ~~~~
 * assert!(all(|&x: &uint| x < 6, |f| uint::range(1, 6, f)));
 * assert!(!all(|&x: &uint| x < 5, |f| uint::range(1, 6, f)));
 * ~~~~
 */
#[inline(always)]
pub fn all<T>(predicate: &fn(T) -> bool, iter: &fn(f: &fn(T) -> bool)) -> bool {
    for iter |x| {
        if !predicate(x) {
            return false
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use prelude::*;

    #[test]
    fn test_iter_to_vec() {
        let xs = ~[1, 2, 3];
        let ys = do iter_to_vec |f| { xs.each(|x| f(*x)) };
        assert_eq!(xs, ys);
    }

    #[test]
    fn test_any() {
        let xs = ~[1u, 2, 3, 4, 5];
        assert!(any(|&x: &uint| x > 2, |f| xs.each(f)));
        assert!(!any(|&x: &uint| x > 5, |f| xs.each(f)));
    }

    #[test]
    fn test_all() {
        assert!(all(|x: uint| x < 6, |f| uint::range(1, 6, f)));
        assert!(!all(|x: uint| x < 5, |f| uint::range(1, 6, f)));
    }
}
