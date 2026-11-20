use std::io;
use std::time::Duration;
use std::sync::Arc;

use crate::state;
use communication::{tool_messages, from_slice_cobs_crc32, to_slice_cobs_crc32};

use tokio_rustls::rustls::pki_types::{CertificateDer, PrivateKeyDer};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use std::net::ToSocketAddrs;
use tokio_rustls::rustls::pki_types::pem::PemObject;
use std::sync::RwLock;

fn add_tool(tools: &RwLock<Vec<RwLock<Option<state::Tool>>>>, new: state::Tool) -> usize {
    // check read before trying to write to anything
    for (i, tool) in tools.read().unwrap().iter().enumerate() {
	if tool.read().unwrap().is_none() {
	    let mut accessor = tool.write().unwrap();
	    if accessor.is_none() {
		*accessor = Some(new);
		return i;
	    }
	}
    }
    
    // couldn't find a space, lock the whole thing to reallocate
    let mut lock = tools.write().unwrap();
    let i = lock.len();
    
    lock.push(RwLock::new(Some(new)));
    while lock.len() < lock.capacity() {
	lock.push(RwLock::new(None));
    }
    
    i
}

pub async fn devices(tools: &RwLock<Vec<RwLock<Option<state::Tool>>>>) -> io::Result<()> {
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

            serve(stream, io_timeout, shutdown_timeout).await?;

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
    tools: &RwLock<Vec<RwLock<Option<state::Tool>>>>,
) -> io::Result<()> {
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

	if let Ok(setup) = from_slice_cobs_crc32::<tool_messages::DeviceSetup>(&mut buf[..n]) {
	    match setup {
		tool_messages::DeviceApikey(apikey) => {
		    let (sender, receiver) = tokio::sync::mpsc::channel(1);
		    let index = add_tool(tools, state::Tool::TubeFurnace(state::TubeFurnace(f64::MIN, 0.0, sender)));
		    serve_tube_furnace(stream, io_timeout, shutdown_timeout, tools, index, receiver).await?;
		}
	    }
	    break;
	} else {
	    // invalid request
	    return Ok(());
	}
	
        //with_timeout(stream.write_all(&buf[..n]), "write", io_timeout).await?;
    }
    
    with_timeout(stream.flush(), "flush", io_timeout).await?;

    // shutdown() writes a close_notify alert, so it too can block on a
    // stalled peer.
    with_timeout(stream.shutdown(), "TLS shutdown", shutdown_timeout).await?;
    Ok(())
}

async fn serve_tube_furnace<IO: AsyncRead + AsyncWrite + Unpin>(
    mut stream: tokio_rustls::server::TlsStream<IO>,
    io_timeout: Duration,
    shutdown_timeout: Duration,
    tools: &RwLock<Vec<RwLock<Option<state::Tool>>>>,
    index: usize,
    receiver: tokio::sync::mpsc::Receiver<()>,
) -> io::Result<()> {
    let mut buf = vec![0u8; 8192];

    loop {
	tokio::select!(
	    _ = receiver.recv() => {
		let lock = tools.read().unwrap()[index].read().unwrap();
		if lock.desired_temp_dirty {
		    let section = to_slice_cobs_crc32(&tool_messages::SetTemp(lock.desired_temp_k), &mut buf)?;
		    with_timeout(stream.write_all(section), "write", io_timeout).await?;
		}
	    },
	    raw = tokio::time::timeout(io_timeout, stream.read(&mut buf)) => {
		let n = match raw {
		Ok(result) => result?,
		Err(_elapsed) => break,
		};
		// and then deal with it
	    }
        );
    }
    
    Ok(())
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
