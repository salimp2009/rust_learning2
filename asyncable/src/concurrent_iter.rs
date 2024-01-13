use futures::Stream;
use std::{io, pin::Pin};

/// for loops are not usable with Streams
pub async fn sum_with_next(mut stream: Pin<&mut dyn Stream<Item = i32>>) -> i32 {
    use futures::stream::StreamExt;

    let mut sum = 0;
    while let Some(item) = stream.next().await {
        sum += item;
    }
    sum
}

pub async fn sum_with_try_next(
    mut stream: Pin<&mut dyn Stream<Item = Result<i32, io::Error>>>,
) -> Result<i32, io::Error> {
    use futures::stream::TryStreamExt;

    let mut sum = 0;
    while let Some(item) = stream.try_next().await? {
        sum += item;
    }

    Ok(sum)
}
