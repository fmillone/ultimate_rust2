use testing::{sploosh, splish};


#[test]
fn sploosh_returns_4_test() {
    let actual = sploosh(
        splish(-1, 0),
        splish(1, 1),
        splish(3, 2)
    );

    assert_eq!(actual, 4);
}