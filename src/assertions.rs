#![feature(macro_rules)]

macro_rules! assert_op(
    ($op:ident ~ $given:expr , $expected:expr) => ({
        match (&($given), &($expected)) {
            (given_val, expected_val) => {
                // check both directions of equality....
                if !((*given_val $op *expected_val) &&
                     (*expected_val $op *given_val)) {
                    panic!("assertion failed: `(left == right) && (right == left)` \
                           (left: `{}`, right: `{}`)", *given_val, *expected_val)
                }
            }
        }
    })
)

#[macro_export]
macro_rules! assert_lt(
    ($given:expr , $expected:expr) => ({
        match (&($given), &($expected)) {
            (given_val, expected_val) => {
                // check both directions of equality....
                if !(*given_val < *expected_val) {
                    panic!("assertion failed: `(left < right)` \
                           (left: `{}`, right: `{}`)", *given_val, *expected_val)
                }
            }
        }
    })
)

#[macro_export]
macro_rules! assert_le(
    ($given:expr , $expected:expr) => ({
        match (&($given), &($expected)) {
            (given_val, expected_val) => {
                // check both directions of equality....
                if !(*given_val <= *expected_val) {
                    panic!("assertion failed: `(left <= right)` \
                           (left: `{}`, right: `{}`)", *given_val, *expected_val)
                }
            }
        }
    })
)

#[macro_export]
macro_rules! assert_ge(
    ($given:expr , $expected:expr) => ({
        match (&($given), &($expected)) {
            (given_val, expected_val) => {
                // check both directions of equality....
                if !(*given_val >= *expected_val) {
                    panic!("assertion failed: `(left >= right)` \
                           (left: `{}`, right: `{}`)", *given_val, *expected_val)
                }
            }
        }
    })
)

#[macro_export]
macro_rules! assert_gt(
    ($given:expr , $expected:expr) => ({
        match (&($given), &($expected)) {
            (given_val, expected_val) => {
                // check both directions of equality....
                if !(*given_val > *expected_val) {
                    panic!("assertion failed: `(left > right)` \
                           (left: `{}`, right: `{}`)", *given_val, *expected_val)
                }
            }
        }
    })
)

