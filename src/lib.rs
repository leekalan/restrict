pub mod non_empty_string;
pub mod owned_restricted_view;
pub mod restrict;

#[cfg(test)]
mod tests {
    use non_empty_string::{NonEmptyStr, NonEmptyString};
    use owned_restricted_view::OwnedRestrictedView;
    use restrict::{Restrict, RestrictView};

    use super::*;

    #[test]
    fn test_original() {
        let _a: &NonEmptyStr;
        let _br;
        {
            let b = "abc".to_string();
            _br = NonEmptyStr::restrict(&b).unwrap();
        }
        // a = br;
    }

    #[test]
    fn test_owned() {
        let string = "abc".to_string();
        let _a = OwnedRestrictedView::<NonEmptyStr>::restrict(string.into_boxed_str()).unwrap();
    }

    #[test]
    fn sizes() {
        println!(
            "{}, {}, {}, {}, {}, {}, {}, {}, {}, {}",
            std::mem::size_of::<&str>(),
            std::mem::size_of::<&NonEmptyStr>(),
            std::mem::size_of::<String>(),
            std::mem::size_of::<NonEmptyString>(),
            std::mem::size_of::<OwnedRestrictedView<NonEmptyStr>>(),
            std::mem::size_of::<Option<&str>>(),
            std::mem::size_of::<Option<&NonEmptyStr>>(),
            std::mem::size_of::<Option<String>>(),
            std::mem::size_of::<Option<NonEmptyString>>(),
            std::mem::size_of::<Option<OwnedRestrictedView<NonEmptyStr>>>()
        );
    }
}
