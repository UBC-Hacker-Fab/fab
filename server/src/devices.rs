use std::io;
use std::time::Duration;
use std::sync::Arc;
use tokio_rustls::rustls::pki_types::{CertificateDer, PrivateKeyDer};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use std::net::ToSocketAddrs;
use tokio_rustls::rustls::pki_types::pem::PemObject;

pub async fn devices() -> io::Result<()> {
    let config = Arc::new(
        tokio_rustls::rustls::ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(
                CertificateDer::pem_file_iter("cert.pem").map_err(ioerror)?.collect::<Result<_, _>>().map_err(ioerror)?,
                PrivateKeyDer::from_pem_file("private.pem").map_err(ioerror)?,
            ).map_err(|err| io::Error::other(err))?,
    );
    
    let addr = "127.0.0.1:9000";
    let addr = addr.to_socket_addrs()?
        .next()
        .ok_or_else(|| io::Error::from(io::ErrorKind::AddrNotAvailable))?;
    
    let handshake_timeout = Duration::from_secs(5);
    let io_timeout = Duration::from_secs(30);
    let shutdown_timeout = Duration::from_secs(5);
    
    let acceptor = tokio_rustls::TlsAcceptor::from(config);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    
    loop {
        let acceptor = acceptor.clone();
        let (stream, peer_addr) = listener.accept().await?;

        let fut = async move {
            let stream = with_timeout(acceptor.accept(stream), "TLS handshake", handshake_timeout).await?;

            let n = serve(stream, io_timeout, shutdown_timeout).await?;
            println!("Echo: {} - {}", peer_addr, n);

            Ok(()) as io::Result<()>
        };

        tokio::spawn(async move {
            if let Err(err) = fut.await {
                eprintln!("{peer_addr}: {err:?}");
            }
        });
    }
}

async fn serve<IO: AsyncRead + AsyncWrite + Unpin>(
    mut stream: tokio_rustls::server::TlsStream<IO>,
    io_timeout: Duration,
    shutdown_timeout: Duration,
) -> io::Result<u64> {
    let mut echoed = 0;
    let mut buf = vec![0u8; 8192];
    loop {
        // Bounding each read() limits how long the peer may sit idle. A
        // single timeout() around the whole loop would bound total
        // connection time instead.
        // For an idle peer, stop echoing and fall through to the graceful
        // shutdown below so the peer sees close_notify rather than an abrupt
        // close.
        let n = match tokio::time::timeout(io_timeout, stream.read(&mut buf)).await {
            Ok(result) => result?,
            Err(_elapsed) => break,
        };
        if n == 0 {
            break;
        }
        with_timeout(stream.write_all(&buf[..n]), "write", io_timeout).await?;
        echoed += n as u64;
    }
    with_timeout(stream.flush(), "flush", io_timeout).await?;

    // shutdown() writes a close_notify alert, so it too can block on a
    // stalled peer.
    with_timeout(stream.shutdown(), "TLS shutdown", shutdown_timeout).await?;
    Ok(echoed)
}

async fn with_timeout<T>(
    fut: impl Future<Output = io::Result<T>>,
    phase: &str,
    duration: Duration,
) -> io::Result<T> {
    match tokio::time::timeout(duration, fut).await {
        Ok(result) => result,
        Err(_) => Err(io::Error::new(
            io::ErrorKind::TimedOut,
            format!("{phase} timed out after {duration:?}"),
        )),
    }
}

fn ioerror(err: tokio_rustls::rustls::pki_types::pem::Error) -> io::Error {
    match err {
	tokio_rustls::rustls::pki_types::pem::Error::Io(ioerror) => ioerror,
	other => io::Error::other(other)
    }
}
