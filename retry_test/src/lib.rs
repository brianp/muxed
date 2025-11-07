use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, LitInt, Token, parse_macro_input, punctuated::Punctuated};

/// Attribute macro for retrying a test function in case of failure or panic.
///
/// # Usage
///
/// Place `#[retry_test]` above a test function to run it multiple times on failure or timeout.
///
/// Optionally, provide parameters to specify the number of retry attempts and a timeout (in seconds) for each attempt:
///
/// - `#[retry_test(N)]` - Retries the test up to `N` times (default: 1).
/// - `#[retry_test(N, T)]` - Retries the test up to `N` times with a timeout of `T` seconds per attempt (default: 10s).
///
/// If the test does not succeed within the specified attempts or time, it panics with an error message.
///
/// This macro can be used alongside other attributes like `#[test]`.
///
/// # Example
///
/// ```no_run
/// #[test]
/// #[retry_test(3, 5)]
/// fn my_unstable_test() {
///     // test code that might intermittently fail
/// }
/// ```
#[proc_macro_attribute]
pub fn retry_test(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse as `1` or `1, 2`
    let args = parse_macro_input!(attr with Punctuated::<LitInt, Token![,]>::parse_terminated);

    let retries: u32 = args
        .first()
        .map(|lit| lit.base10_parse().unwrap())
        .unwrap_or(1);
    let timeout_secs: u64 = args
        .get(1)
        .map(|lit| lit.base10_parse().unwrap())
        .unwrap_or(10);

    let input = parse_macro_input!(item as ItemFn);

    let vis = &input.vis;
    let sig = &input.sig;
    let block = &input.block;

    // keep other attrs (e.g. #[test]) but drop #[retry_test]
    let attrs: Vec<_> = input
        .attrs
        .into_iter()
        .filter(|a| !a.path().is_ident("retry_test"))
        .collect();

    let expanded = quote! {
        #(#attrs)*
        #vis #sig {
            let mut attempts: u32 = 0;
            let max_attempts: u32 = #retries;

            while attempts < max_attempts {
                let (tx, rx) = ::std::sync::mpsc::channel();
                ::std::thread::spawn(move || {
                    let res = ::std::panic::catch_unwind(
                        ::std::panic::AssertUnwindSafe(|| { #block })
                    );
                    let _ = tx.send(res);
                });

                match rx.recv_timeout(::std::time::Duration::from_secs(#timeout_secs)) {
                    Ok(Ok(())) => return, // passed
                    Ok(Err(_)) => {
                        eprintln!("Attempt {} panicked", attempts + 1);
                    }
                    Err(::std::sync::mpsc::RecvTimeoutError::Timeout) => {
                        eprintln!(
                            "Attempt {} timed out after {}s",
                            attempts + 1,
                            #timeout_secs
                        );
                    }
                    Err(_) => {
                        eprintln!("Attempt {} failed due to channel error", attempts + 1);
                    }
                }

                attempts += 1;
            }

            panic!(
                "Test failed after {} attempts ({}s timeout each).",
                max_attempts,
                #timeout_secs
            );
        }
    };

    TokenStream::from(expanded)
}
