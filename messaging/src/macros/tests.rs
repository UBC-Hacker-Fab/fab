use super::{messages, protocol};
use serde::{Deserialize, Serialize};

messages! {
    pub struct A {
        a: u8,
    }

    pub struct B {
        b: u8,
    }

    pub struct C {
        c: u8,
    }

    pub struct D {
        d: u8,
    }

    pub struct E {
        e: u8,
    }
}

protocol!(TestFrame { A, B, C, D, E });

fn sample_frames() -> [(TestFrame, &'static str, usize); 5] {
    [
        (TestFrame::A(A { a: 1 }), "A", 0),
        (TestFrame::B(B { b: 2 }), "B", 1),
        (TestFrame::C(C { c: 3 }), "C", 2),
        (TestFrame::D(D { d: 4 }), "D", 3),
        (TestFrame::E(E { e: 5 }), "E", 4),
    ]
}

#[test]
fn count_is_correct() {
    assert_eq!(TestFrame::COUNT, 5);
}

#[test]
fn names_match_expected() {
    for (frame, expected_name, _) in sample_frames() {
        assert_eq!(frame.name(), expected_name);
    }
}

#[test]
fn indexes_match_declaration_order() {
    for (frame, _, expected_index) in sample_frames() {
        assert_eq!(frame.index(), expected_index);
    }

    assert_eq!(<A as TestFrameTrait>::INDEX, 0);
    assert_eq!(<B as TestFrameTrait>::INDEX, 1);
    assert_eq!(<C as TestFrameTrait>::INDEX, 2);
    assert_eq!(<D as TestFrameTrait>::INDEX, 3);
    assert_eq!(<E as TestFrameTrait>::INDEX, 4);
}

#[test]
fn serialization_round_trips() {
    let alpha = A { a: 1 };
    let mut buf = [0u8; 32];
    let bytes = postcard::to_slice(&alpha, &mut buf).unwrap();
    let decoded: A = postcard::from_bytes(bytes).unwrap();
    assert_eq!(decoded, alpha);

    for (frame, _, _) in sample_frames() {
        let mut buf = [0u8; 32];
        let bytes = postcard::to_slice(&frame, &mut buf).unwrap();
        let decoded: TestFrame = postcard::from_bytes(bytes).unwrap();
        assert_eq!(decoded, frame);
    }
}

#[test]
fn into_frame_and_back_round_trips() {
    let alpha = A { a: 1 };
    let frame: TestFrame = alpha.clone().into();
    assert_eq!(A::try_from(frame), Ok(alpha));

    let bravo = B { b: 2 };
    let frame: TestFrame = bravo.clone().into();
    assert_eq!(B::try_from(frame), Ok(bravo));

    let charlie = C { c: 3 };
    let frame: TestFrame = charlie.clone().into();
    assert_eq!(C::try_from(frame), Ok(charlie));

    let delta = D { d: 4 };
    let frame: TestFrame = delta.clone().into();
    assert_eq!(D::try_from(frame), Ok(delta));

    let echo = E { e: 5 };
    let frame: TestFrame = echo.clone().into();
    assert_eq!(E::try_from(frame), Ok(echo));
}
