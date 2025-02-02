mod time {
    use bstr::ByteSlice;
    use git_object::{Sign, Time};

    #[test]
    fn write_to() -> Result<(), Box<dyn std::error::Error>> {
        for (time, expected) in &[
            (
                Time {
                    time: 500,
                    offset: 9000,
                    sign: Sign::Plus,
                },
                "500 +0230",
            ),
            (
                Time {
                    time: 189009009,
                    offset: 36000,
                    sign: Sign::Minus,
                },
                "189009009 -1000",
            ),
            (
                Time {
                    time: 0,
                    offset: 0,
                    sign: Sign::Minus,
                },
                "0 -0000",
            ),
        ] {
            let mut output = Vec::new();
            time.write_to(&mut output)?;
            assert_eq!(output.as_bstr(), expected);
        }
        Ok(())
    }
}

mod signature {
    mod write_to {
        mod invalid {
            use git_object::{mutable, Sign, Time};

            #[test]
            fn name() {
                let signature = mutable::Signature {
                    name: "invalid < middlename".into(),
                    email: "ok".into(),
                    time: default_time(),
                };
                assert_eq!(
                    format!("{:?}", signature.write_to(Vec::new())),
                    "Err(Custom { kind: Other, error: IllegalCharacter })"
                );
            }

            #[test]
            fn email() {
                let signature = mutable::Signature {
                    name: "ok".into(),
                    email: "server>.example.com".into(),
                    time: default_time(),
                };
                assert_eq!(
                    format!("{:?}", signature.write_to(Vec::new())),
                    "Err(Custom { kind: Other, error: IllegalCharacter })"
                );
            }

            #[test]
            fn name_with_newline() {
                let signature = mutable::Signature {
                    name: "hello\nnewline".into(),
                    email: "name@example.com".into(),
                    time: default_time(),
                };
                assert_eq!(
                    format!("{:?}", signature.write_to(Vec::new())),
                    "Err(Custom { kind: Other, error: IllegalCharacter })"
                );
            }

            fn default_time() -> Time {
                Time {
                    time: 0,
                    offset: 0,
                    sign: Sign::Plus,
                }
            }
        }
    }

    use bstr::ByteSlice;
    use git_object::{immutable, mutable};

    #[test]
    fn round_trip() -> Result<(), Box<dyn std::error::Error>> {
        for input in &[
            &b"Sebastian Thiel <byronimo@gmail.com> 1 -0030"[..],
            ".. ☺️Sebastian 王知明 Thiel🙌 .. <byronimo@gmail.com> 1528473343 +0230".as_bytes(),
            ".. whitespace  \t  is explicitly allowed    - unicode aware trimming must be done elsewhere <byronimo@gmail.com> 1528473343 +0230".as_bytes(),
        ] {
            let signature: mutable::Signature = immutable::Signature::from_bytes(input)?.into();
            let mut output = Vec::new();
            signature.write_to(&mut output)?;
            assert_eq!(output.as_bstr(), input.as_bstr());
        }
        Ok(())
    }
}

use git_object::mutable::Object;

#[test]
fn size_in_memory() {
    assert_eq!(
        std::mem::size_of::<Object>(),
        264,
        "Prevent unexpected growth of what should be lightweight objects"
    )
}
