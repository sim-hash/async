mod ffi;
mod poll;
mod bitmask;


use ffi::Event;
use mio::{Token, Interest};
use poll::Poll;
use std::{io::{Write, Result, Read, ErrorKind, self}, net::TcpStream, env, collections::HashSet, usize};

fn main() -> Result<()> {
    poll_events_mio_impl();
    Ok(())
}

fn poll_events_mio_impl() ->Result<()> {
    let mut poll = mio::Poll::new()?;
    let n_events = 5;
 
    let mut streams = vec![];
    let base_url = env::args().nth(1).unwrap_or_else(|| String::from("localhost"));

    let addr = format!("{}:8080", &base_url);
    for i in 0..n_events {
        let delay = (n_events - 1) * 1000;
        let url_path = format!("/{delay}/request-{i}");
        let request = get_req(&url_path);
        let std_stream = std::net::TcpStream::connect(&addr)?;
        std_stream.set_nonblocking(true)?;
         
        let mut stream = mio::net::TcpStream::from_std(std_stream);
        stream.write_all(request.as_bytes())?;

        poll.registry().register(&mut stream, Token(i), Interest::READABLE)?;
        streams.push(stream);
    }

    let mut handled_ids = HashSet::new();
    let mut handled_events = 0;

    while handled_events < n_events {
        let mut events = mio::Events::with_capacity(10);
        poll.poll(&mut events, None);
        
        if events.is_empty() {
            println!("TIMEOUT (OR SPURIOUS EVENT NOTIFICATION)");
            continue;
        }

        let events: Vec<mio::event::Event> = events.into_iter().map(|e| e.clone()).collect();
        handled_events += handle_events_mio_impl(&events, &mut streams, &mut handled_ids)?;
    }

    println!("Finished");
    Ok(())
}

fn handle_events_mio_impl(events: &[mio::event::Event], streams: &mut [mio::net::TcpStream], handled: &mut HashSet<usize>) -> Result<usize> {
    let mut handled_events = 0;
    for event in events {
        // extract value wrapped in token
        let index: usize = event.token().into();
        let mut data = vec![0u8; 4096];
        loop {
            match streams[index].read(&mut data) {
                Ok(n) if n == 0 => {
                    if !handled.insert(index) {
                        break;
                    }

                    handled_events += 1;
                    break
                }
                Ok(n) => {
                    let txt = String::from_utf8_lossy(&data[..n]);

                    println!("RECEIVED: {:?}", event);
                    println!("{txt}\n--------\n");
                }
                Err(e) if matches!(e.kind(), io::ErrorKind::WouldBlock | io::ErrorKind::Interrupted) => break,
                Err(e) => return Err(e),
            }
        }
    }

    Ok(handled_events)
}

fn poll_events_standard_implementation() -> Result<()> {
    let mut poll = Poll::new()?;
    let n_events = 5;

    let mut streams = vec![];
    let addr = "localhost:8080";

    for i in 0..n_events {
        let delay = (n_events - 1) * 1000;
        let url_path = format!("/{delay}/request-{i}");
        let request  = get_req(&url_path);
        let mut stream = std::net::TcpStream::connect(addr)?;
        stream.set_nonblocking(true)?;

        stream.write_all(request.as_bytes())?;
        poll.registry().register(&stream, i, ffi::EPOLLIN | ffi::EPOLLET)?;
        streams.push(stream);

    }

    let mut handled_events = 0;
    while handled_events < n_events {
        let mut events = Vec::with_capacity(10);
        poll.poll(&mut events, None)?;

        if events.is_empty() {
            println!("TIMEOUT OR (SERIOUS EVENT NOTIFICATION)");
            continue;
        }

        handled_events += handle_events_std_impl(&events, &mut streams)?;
    }

    Ok(())
}

fn handle_events_std_impl(events: &[Event], streams: &mut [TcpStream]) -> Result<usize> {
    let mut handled_events = 0;

    for event in events {
        let index = event.token();
        let mut data = vec![0u8; 4096];

        loop {
            match streams[index].read(&mut data) {
                Ok(n) if n == 0 => {
                    handled_events += 1;
                    break;
                }
                Ok(n) => {
                    let txt = String::from_utf8_lossy(&data[..n]);
                    println!("RECEIVED: {:?}", event);
                    println!("{txt}\n-----\n");
                }
                Err(e) if e.kind() == ErrorKind::WouldBlock => break,
                Err(e) => return Err(e),
            }
        }
    }
    Ok(handled_events)
}

fn get_req(path: &str) -> String {
    format!(
        "GET {path} HTTP/1.1\r\n\
        Host: localhost\r\n\
        Connection: close\r\n\
        \r\n"
        )
}

fn test_syscall() {

  let message = String::from("This is my first run for rust async");
  syscall(message);

}

#[cfg(target_os="linux")]
#[inline(never)]
fn syscall(message: String) {
    use std::arch::asm;

    let msg_ptr = message.as_ptr();
    let len = message.len();

    println!("{:?}", msg_ptr);

    unsafe {
        asm!(
            "mov rax, 1",
            "mov rdi, 1",
            "syscall",
            in("rsi") msg_ptr,
            in("rdx") len,
            out("rax") _,
            out("rdi") _,
            lateout("rsi") _,
            lateout("rdx") _,
            );
    }
}
