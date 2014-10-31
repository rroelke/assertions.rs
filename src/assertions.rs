#![feature(macro_rules)]

macro_rules! assert_op(
    ($op:ident ~ $given:expr , $expected:expr) => ({
        match (&($given), &($expected)) {
            (given_val, expected_val) => {
                // check both directions of equality....
                if !((*given_val $op *expected_val) &&
                     (*expected_val $op *given_val)) {
                    fail!("assertion failed: `(left == right) && (right == left)` \
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
                    fail!("assertion failed: `(left < right)` \
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
                    fail!("assertion failed: `(left <= right)` \
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
                    fail!("assertion failed: `(left >= right)` \
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
                    fail!("assertion failed: `(left > right)` \
                           (left: `{}`, right: `{}`)", *given_val, *expected_val)
                }
            }
        }
    })
)

#[test]
fn test_assert_lt() {
    assert_lt!(0u, 1u);

    let mut result : Result<(), Box<std::any::Any + Send>> = std::task::try(proc() {
        assert_lt!(0u, 0u)
    });
    assert!(result.is_err());

    result = std::task::try(proc() {
        assert_lt!(1u, 0u)
    });
    assert!(result.is_err());
}

#[test]
fn test_assert_le() {
    assert_le!(0u, 1u);
    assert_le!(0u, 0u);

    let result = std::task::try(proc() {
        assert_le!(1u, 0u)
    });
    assert!(result.is_err());
}

#[test]
fn test_assert_ge() {
    assert_ge!(1u, 0u);
    assert_ge!(0u, 0u);

    let result = std::task::try(proc() {
        assert_ge!(0u, 1u)
    });
    assert!(result.is_err());
}

#[test]
fn test_assert_gt() {
    assert_gt!(1u, 0u);

    let mut result : Result<(), Box<std::any::Any + Send>> = std::task::try(proc() {
        assert_gt!(0u, 0u)
    });
    assert!(result.is_err());

    result = std::task::try(proc() {
        assert_gt!(0u, 1u)
    });
    assert!(result.is_err());
}
