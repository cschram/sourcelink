use crate::error::SourcelinkError;
use anyhow::Result;

pub fn substr(s: &str, start: usize, len: usize) -> Result<String> {
    if (start + len) > s.len() {
        Err(SourcelinkError::SubstrRange.into())
    } else {
        Ok(s.chars().skip(start).take(len).collect())
    }
}

pub fn substr_eq(left: &str, start: usize, right: &str) -> bool {
    if (start + right.len()) > left.len() {
        false
    } else {
        substr(left, start, right.len()).unwrap() == right
    }
}

#[cfg(test)]
mod test {
    use crate::error::SourcelinkError;

    #[test]
    fn substr() {
        {
            let ss = super::substr("abcdef", 0, 3);
            assert!(ss.is_ok());
            assert_eq!(&ss.unwrap(), "abc");
        }
        {
            let ss = super::substr("abcdef", 2, 4);
            assert!(ss.is_ok());
            assert_eq!(&ss.unwrap(), "cdef");
        }
        {
            let ss = super::substr("abcdef", 3, 4);
            assert!(ss.is_err());
            assert!(matches!(
                ss.unwrap_err().downcast_ref::<SourcelinkError>(),
                Some(SourcelinkError::SubstrRange)
            ));
        }
    }

    #[test]
    fn substr_eq() {
        assert!(super::substr_eq("abcdef", 0, "abc"));
        assert!(super::substr_eq("abcdef", 2, "cdef"));
        assert!(!super::substr_eq("abcdef", 4, "ab"));
        assert!(!super::substr_eq("abcdef", 5, "fghifk"));
    }
}
