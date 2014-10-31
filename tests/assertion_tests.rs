#![feature(phase)]
#[phase(plugin)]

extern crate assertions;

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

