#![forbid(unsafe_code)]
use cfg_if::cfg_if;
use std::future::Future;

/// Spawns and runs a thread-local [`Future`] in a platform-independent way.
///
/// This can be used to interface with any `async` code.
pub fn spawn_local<F>(fut: F)
where
    F: Future<Output = ()> + 'static,
{
    cfg_if! {
        if #[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))] {
            // worker::console_log!("Spawned Local");
            // let f = async move{
            //     worker::console_log!("Started Local");
            //     fut.await;
            //     worker::console_log!("Ended Local");
            // };

            wasm_bindgen_futures::spawn_local(fut);
        }else if #[cfg(all(target_arch = "wasm32", feature = "ssr"))] {
            wasm_bindgen_futures::spawn_local(fut);
        }
        else if #[cfg(any(test, doctest))] {
            tokio_test::block_on(fut);
        } else if #[cfg(all(feature = "ssr", not(target_arch = "wasm32")))] {
            tokio::task::spawn_local(fut);
        }  else {
            futures::executor::block_on(fut)
        }
    }
}
