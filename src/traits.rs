use std::{future::Future, pin::Pin};

use async_trait::async_trait;

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

#[async_trait]
/// AsyncMap is a trait used to map a value under asynchronous contexts.
///
/// It allows you to pass in a boxed + pinned future and perform activites similar to std's map for Options/Results.
pub trait AsyncMap<T, U, F>
where
    F: FnOnce(T) -> Pin<Box<dyn Future<Output = U> + Send>> + Send,
{
    type Output;
    async fn async_map(self, map: F) -> Self::Output;
}

#[async_trait]
impl<T, U, F> AsyncMap<T, U, F> for Option<T>
where
    T: Send,
    U: Send,
    F: 'static + FnOnce(T) -> Pin<Box<dyn Future<Output = U> + Send>> + Send,
{
    type Output = Option<U>;
    async fn async_map(self, map: F) -> Self::Output {
        match self {
            Some(t) => {
                let u = map(t).await;
                Some(u)
            }
            None => None,
        }
    }
}

#[async_trait]
impl<T, E, U, F> AsyncMap<T, U, F> for Result<T, E>
where
    T: Send,
    U: Send,
    E: Send,
    F: 'static + FnOnce(T) -> Pin<Box<dyn Future<Output = U> + Send>> + Send,
{
    type Output = Result<U, E>;
    async fn async_map(self, map: F) -> Self::Output {
        match self {
            Ok(t) => {
                let u = map(t).await;
                Ok(u)
            }
            Err(e) => Err(e),
        }
    }
}
