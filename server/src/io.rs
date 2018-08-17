use core::errors::Error;
use futures::{Future, Stream};
use futures_cpupool::CpuPool;
use std::fs::File;
use std::io::Write;
use std::ops::Deref;
use std::sync::Arc;

/// ## Stream the content of the stream to a file
///
/// Delegates all blocking I/O to a CpuPool.
pub fn stream_to_file<S, C, E>(
    file: File,
    pool: Arc<CpuPool>,
    stream: S,
) -> Box<Future<Item = File, Error = Error>>
where
    S: 'static + Stream<Item = C, Error = E>,
    C: 'static + Send + Deref<Target = [u8]>,
    E: 'static + Into<Error>,
{
    // Write file in chunks as it becomes available
    let out = stream
        .map_err(Into::into)
        .fold((pool, file), |(pool, mut file), chunk| {
            // Write chunks on cpu-pool
            pool.spawn_fn(move || match file.write_all(chunk.as_ref()) {
                Ok(_) => Ok(file),
                Err(e) => Err(e),
            }).map(move |file| (pool, file))
        }).map(|(_, file)| file)
        .map_err(Into::into);

    Box::new(out)
}
