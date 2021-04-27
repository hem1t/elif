#[macro_export]
macro_rules! elif {
    (
        $(($ifcond:expr) $if:expr)+,
        $(($elifcond:expr) $elif:expr), *,
        $(() $else:expr;)?
    ) => {
        // if statement
        $(
            if $ifcond {
                $if
            }
        )+
        // else if statements
        $(
            else if $elifcond {
                $elif
            }
        )*
        // else statement
        $(
            else {
                $else
            }
        )?
    }
}

#[allow(dead_code)]
fn num(n: u8) -> &'static str {
    elif! {
        (n == 1) "One",
        (n == 2) "Two",
        (n == 3) "Wednesday",
        (n == 4) "4",
        () "None";
    }
}

// #[allow(dead_code)]
// fn big()

#[test]
fn num_test_1() {
    assert_eq!(num(1), "One");
    assert_eq!(num(2), "Two");
    assert_eq!(num(3), "Wednesday");
    assert_eq!(num(4), "4");
    assert_eq!(num(10), "None");
}
