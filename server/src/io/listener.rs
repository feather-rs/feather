use super::*;
use mio::{
    net::{TcpListener, TcpStream},
    Events, Poll, PollOpt, Ready, Token,
};

const SERVER: Token = Token(0);
const MESSAGE_RECEIVER: Token = Token(1);

/// Starts a listener in the current thread,
/// blocking indefinitely until a`ShutDown` message
/// is received.
pub fn start(
    address: String,
    sender: Sender<ServerToListenerMessage>,
    receiver: Receiver<ServerToListenerMessage>,
    worker_senders: Vec<Sender<ListenerToWorkerMessage>>,
) {
    let poll = Poll::new().unwrap();

    let server = TcpListener::bind(&address.parse().unwrap()).unwrap();

    poll.register(&server, SERVER, Ready::readable(), PollOpt::edge())
        .unwrap();

    let mut workers = vec![];

    for (i, sender) in worker_senders.into_iter().enumerate() {
        let worker = Worker {
            sender,
        };
        workers.push(worker);
    }

    let mut events = Events::with_capacity(1024);
    let mut next_worker_index = 0;

    loop {
        poll.poll(&mut events, None).unwrap();

        for event in &events {
            if event.token() == SERVER {
                // New connection
                if let Ok((stream, addr)) = server.accept() {
                    info!("Accepting connection from {}", addr);
                    debug!(
                        "Connection will be handled by worker #{}",
                        next_worker_index
                    );

                    let message = ListenerToWorkerMessage::NewConnection(stream, addr);

                    workers[next_worker_index].sender.send(message).unwrap();

                    if next_worker_index == workers.len() - 1 {
                        next_worker_index = 0;
                    } else {
                        next_worker_index += 1;
                    }
                }
            } else if event.token() == MESSAGE_RECEIVER {
                // Message from server
                if let Ok(message) = receiver.try_recv() {
                    match message {
                        ServerToListenerMessage::ShutDown => {
                            for worker in workers.iter() {
                                worker.sender.send(ListenerToWorkerMessage::ShutDown).unwrap();
                            }
                            return;
                        }
                        _ => panic!("IO listener received invalid message from server"),
                    }
                }
            }
        }
    }
}

struct Worker {
    sender: Sender<ListenerToWorkerMessage>,
}
