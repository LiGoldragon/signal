//! The asynchronous side of the Signal frame, for Nexus transports.

use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

use crate::frame::{
    FRAME_PREFIX_BYTES, Framable, FrameBody, FrameCapacity, FrameError, FramePrefix,
    LengthDeclaring,
};

/// An asynchronous byte source yields one whole Signal frame at a time.
pub trait AsyncFrameReading {
    fn read_frame(
        &mut self,
        capacity: FrameCapacity,
    ) -> impl std::future::Future<Output = Result<FrameBody, FrameError>> + Send;
}

impl<R: AsyncRead + Unpin + Send> AsyncFrameReading for R {
    // Written as a manual future rather than `async fn`: the returned future
    // must be `Send` so a Nexus can spawn a connection that holds it.
    #[allow(clippy::manual_async_fn)]
    fn read_frame(
        &mut self,
        capacity: FrameCapacity,
    ) -> impl std::future::Future<Output = Result<FrameBody, FrameError>> + Send {
        async move {
            let mut prefix = [0_u8; FRAME_PREFIX_BYTES];
            self.read_exact(&mut prefix).await?;
            let length = FramePrefix::from(prefix).admitted_length(capacity)?;
            let mut bytes = vec![0_u8; length];
            self.read_exact(&mut bytes).await?;
            Ok(FrameBody::from(bytes))
        }
    }
}

/// An asynchronous byte sink accepts one whole Signal frame at a time.
pub trait AsyncFrameWriting {
    fn write_frame(
        &mut self,
        body: &(impl Framable + Sync),
        capacity: FrameCapacity,
    ) -> impl std::future::Future<Output = Result<(), FrameError>> + Send;
}

impl<W: AsyncWrite + Unpin + Send> AsyncFrameWriting for W {
    // Written as a manual future rather than `async fn`, for the same reason.
    #[allow(clippy::manual_async_fn)]
    fn write_frame(
        &mut self,
        body: &(impl Framable + Sync),
        capacity: FrameCapacity,
    ) -> impl std::future::Future<Output = Result<(), FrameError>> + Send {
        async move {
            let frame = body.framed(capacity)?;
            self.write_all(&frame).await?;
            self.flush().await?;
            Ok(())
        }
    }
}
