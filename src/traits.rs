/// Discard is a trait used to discard the values of results/options.
///
/// This trait provides a single method, `discard`, which is particularly useful if you want to run the result/option code, but don't care about the returned result.
pub trait Discard {
    /// Takes ownership of the Result/Option and discards the value.
    ///
    /// # Example
    /// ```rust
    /// use rustility::Discard;
    /// fn code_that_returns_a_result() -> Result<i32, Box<dyn std::error::Error>> {
    ///     let computation_value = 1;
    ///     assert_eq!(computation_value, 1);
    ///     Ok(computation_value)
    /// }
    /// code_that_returns_a_result().discard();
    ///
    /// ```
    fn discard(self);
}

impl<T, E> Discard for Result<T, E> {
    fn discard(self) {
        let _ = self;
        drop(self);
    }
}

impl<T> Discard for Option<T> {
    fn discard(self) {
        let _ = self;
        drop(self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Error, ErrorKind};

    fn code_that_returns_ok() -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn code_that_returns_err() -> Result<(), Box<dyn std::error::Error>> {
        Err(Box::new(Error::new(ErrorKind::Other, "Random Error")))
    }

    fn code_that_returns_some() -> Option<()> {
        Some(())
    }

    fn code_that_returns_none() -> Option<()> {
        None
    }

    #[test]
    fn test_discard() {
        code_that_returns_ok().discard();
    }

    #[test]
    fn test_discard_err() {
        code_that_returns_err().discard();
    }

    #[test]
    fn test_discard_some() {
        code_that_returns_some().discard();
    }

    #[test]
    fn test_discard_none() {
        code_that_returns_none().discard();
    }
}
