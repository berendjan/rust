//! Linux-specific extensions to primitives in the [`std::net`] module.
//!
//! [`std::net`]: crate::net

#[unstable(feature = "tcp_quickack", issue = "96256")]

use crate::io;
use crate::net;
use crate::sealed::Sealed;

/// Os-specific extensions for [`TcpStream`]
///
/// [`TcpStream`]: net::TcpStream
#[unstable(feature = "tcp_quickack", issue = "96256")]
pub trait TcpStreamExt: Sealed {
    /// Sets the value of the `TCP_QUICKACK` option on this socket.
    ///
    /// If set, acks are sent immediately, rather than delayed if needed in
    /// accordance to normal TCP operation. This flag is not permanent, it
    /// only enables a switch to or from quickack mode.  Subsequent operation
    /// of the TCP protocol will once again enter/leave quickack mode
    /// depending on internal protocol processing and other factors.
    /// (since Linux 2.4.4)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// #![feature(tcp_quickack)]
    /// use std::net::TcpStream;
    ///
    /// let stream = TcpStream::connect("127.0.0.1:8080")
    ///            .expect("Couldn't connect to the server...");
    /// stream.set_quickack(true).expect("set_quickack call failed");
    /// ```
    #[unstable(feature = "tcp_quickack", issue = "96256")]
    fn set_quickack(&self, quickack: bool) -> io::Result<()>;

    /// Gets the value of the `TCP_QUICKACK` option on this socket.
    ///
    /// For more information about this option, see [`TcpStream::set_quickack`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// #![feature(tcp_quickack)]
    /// use std::net::TcpStream;
    ///
    /// let stream = TcpStream::connect("127.0.0.1:8080")
    ///            .expect("Couldn't connect to the server...");
    /// stream.set_quickack(true).expect("set_quickack call failed");
    /// assert_eq!(stream.quickack().unwrap_or(false), true);
    /// ```
    #[unstable(feature = "tcp_quickack", issue = "96256")] 
    fn quickack(&self) -> io::Result<bool>;
}

impl Sealed for net::TcpStream {}

impl TcpStreamExt for net::TcpStream { 
    fn set_quickack(&self, quickack: bool) -> io::Result<()> {
        self.as_inner().set_quickack(quickack)
    }

    fn quickack(&self) -> io::Result<bool> {
        self.as_inner().quickack()
    }
}