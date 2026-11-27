//! SHACL languageIn parameters use RFC 4647 basic ranges, not RFC 5646 tags.

use shacl_structural_model::LanguageTag;

#[test]
fn basic_ranges_accept_wildcards_and_non_tag_subtag_sequences() {
    for lexical in ["*", "EN", "en-GB", "de-1996-1996", "en-a-foo-a-bar", "a-1"] {
        let range = LanguageTag::try_new(lexical).unwrap();
        assert_eq!(range.as_str(), lexical);
        let json = serde_json::to_string(&range).unwrap();
        assert_eq!(serde_json::from_str::<LanguageTag>(&json).unwrap(), range);
        let binary = postcard::to_allocvec(&range).unwrap();
        assert_eq!(postcard::from_bytes::<LanguageTag>(&binary).unwrap(), range);
    }
}

#[test]
fn malformed_basic_ranges_fail_in_constructors_and_deserializers() {
    for lexical in [
        "",
        "en--GB",
        "-en",
        "en-",
        "123",
        "en-*",
        "abcdefghi",
        "en-123456789",
    ] {
        assert!(LanguageTag::try_new(lexical).is_err(), "{lexical}");
        let json = serde_json::to_string(lexical).unwrap();
        assert!(
            serde_json::from_str::<LanguageTag>(&json).is_err(),
            "{lexical}"
        );
        let binary = postcard::to_allocvec(lexical).unwrap();
        assert!(
            postcard::from_bytes::<LanguageTag>(&binary).is_err(),
            "{lexical}"
        );
    }
}
