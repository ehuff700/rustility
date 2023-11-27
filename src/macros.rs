
#[macro_export]
/// Creates a wrapper closure/async block as needed around a block or an expression.
///
/// This is useful within the contexts of functions that do not return a result or an option. In some cases, you use a lot of `?` on your result/option types, which isn't supported by functions that don't return a result or an option.
/// This is highly problematic and results in a lot of `match`, `if let` statements, or repetitive functional chains like `is_some_and() ....`
///
/// In those cases, you can use this macro to wrap that block/expression and return the results, matching on the final result/option as needed.
///
/// Some would argue that you could create a simple function that returns a result or an option, but that is a lot of boilerplate code when you may need to use something like this multiple times throughout the application.
///
/// # Example
/// ```rust
/// use rustility::result_or_option;
///
/// let r: Result<i32, Box<dyn std::error::Error>> = result_or_option!({
///     let x = 5;
///     let y = 10;
///     // Add more code here, propagating errors with `?`.
///     Ok(x + y)
/// });
///
/// match r {
///     Ok(x) => {
///         assert_eq!(x, 15);
///         println!("sum was: {}", x)
///     },
///     Err(e) => println!("error: {}", e),
/// };
/// ```
macro_rules! result_or_option {
    // Asynchronous block
    (async $e:block) => {
        async $e.await
    };
    // Asynchronous expression
    (async $e:expr) => {
        async { $e }.await
    };
    ($e:block) => {
        (|| $e)()
    };
    ($e:expr) => {
        (|| $e)()
    };
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use std::io::Error as IoError;
    use std::io::ErrorKind;

    enum ResultKind {
        Ok,
        Err,
    }

    enum OptionKind {
        Some,
        None,
    }

    async fn async_result_helper(
        result_kind: ResultKind,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        result_helper(result_kind)
    }

    async fn async_option_helper(option_kind: OptionKind) -> Option<i32> {
        option_helper(option_kind)
    }

    fn result_helper(result_kind: ResultKind) -> Result<i32, Box<dyn std::error::Error>> {
        match result_kind {
            ResultKind::Ok => {
                let computation_value = 1;
                assert_eq!(computation_value, 1);
                Ok(1)
            }
            ResultKind::Err => Err(Box::new(IoError::new(ErrorKind::Other, "Random Error"))),
        }
    }

    fn option_helper(option_kind: OptionKind) -> Option<i32> {
        match option_kind {
            OptionKind::Some => {
                let async_computation_value = 1;
                assert_eq!(async_computation_value, 1);
                Some(1)
            }
            OptionKind::None => None,
        }
    }

    #[test]
    fn test_result() {
        // Test Block
        let r: Result<i32, Box<dyn std::error::Error>> =
            result_or_option!({ result_helper(ResultKind::Ok) });
        assert!(r.is_ok_and(|v| v == 1));

        // Test expr
        let r1: Result<i32, Box<dyn std::error::Error>> =
            result_or_option!(result_helper(ResultKind::Err));
        assert!(r1.is_err());
    }

    #[tokio::test]
    async fn test_async_result() {
        use std::error::Error;
        // Test Block
        let r: Result<i32, Box<dyn Error>> =
            result_or_option!(async { async_result_helper(ResultKind::Ok).await });
        assert!(r.is_ok_and(|v| v == 1));

        // Test expr
        let r1: Result<i32, Box<dyn Error>> =
            result_or_option!(async async_result_helper(ResultKind::Err).await);
        assert!(r1.is_err());
    }

    #[test]
    fn test_option() {
        // Test Block
        let r: Option<i32> = result_or_option!({ option_helper(OptionKind::Some) });
        assert_eq!(r, Some(1));

        // Test expr
        let r1: Option<i32> = result_or_option!(option_helper(OptionKind::None));
        assert!(r1.is_none());
    }

    #[tokio::test]
    async fn test_async_option() {
        // Test Block
        let r: Option<i32> =
            result_or_option!(async { async_option_helper(OptionKind::Some).await });
        assert!(r.is_some_and(|v| v == 1));

        // Test expr
        let r1: Option<i32> = result_or_option!(async async_option_helper(OptionKind::None).await);
        assert!(r1.is_none());
    }
}
