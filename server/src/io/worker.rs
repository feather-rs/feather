use super::*;
use feather_core::bytebuf::ByteBuf;
use feather_core::network::serialize::ConnectionIOManager;
use mio::Event;
use mio::{net::TcpStream, Events, Poll, PollOpt, Ready, Token};
use std::collections::HashMap;
use std::io::Read;
use std::io::Write;
use bytes::BufMut;

// The token used to listen on the channel receiving messages from the listener thread
const LISTENER_TOKEN: Token = Token(0);

struct Worker {
    poll: Poll,
    receiver: Receiver<ListenerToWorkerMessage>,
    sender: Sender<ListenerToWorkerMessage>,
    running: bool,
    client_id_counter: usize,

    clients: HashMap<Client, ClientHandle>,
}

struct ClientHandle {
    stream: TcpStream,
    addr: SocketAddr,

    write_buffer: Option<ByteBuf>,

    receiver: Receiver<ServerToWorkerMessage>,
    sender: Sender<ServerToWorkerMessage>,

    stream_token: Token,
    server_to_worker_token: Token,

    manager: ConnectionIOManager,
}

/// Starts an IO worker on the current thread,
/// blocking indefinitely until a `ShutDown` message
/// is received from the listener.
pub fn start(receiver: Receiver<ListenerToWorkerMessage>, sender: Sender<ListenerToWorkerMessage>) {
    trace!("Starting IO worker thread");
    let poll = Poll::new().unwrap();

    let mut worker = Worker {
        poll,
        receiver,
        sender,
        running: true,

        client_id_counter: 0,
        clients: HashMap::new(),
    };

    worker.poll.register(
        &worker.receiver,
        LISTENER_TOKEN,
        Ready::readable(),
        PollOpt::edge()
    ).unwrap();

    run_loop(&mut worker);
}

fn run_loop(worker: &mut Worker) {
    let mut events = Events::with_capacity(1024);
    while worker.running {
        worker.poll.poll(&mut events, None).unwrap();

        for event in &events {
            trace!("Handling event with readiness {:?} and token {:?}", event.readiness(), event.token());
            handle_event(worker, event);
        }
    }
}

fn handle_event(worker: &mut Worker, event: Event) {
    let readiness = event.readiness();
    if readiness.is_readable() {
        match event.token() {
            LISTENER_TOKEN => read_from_listener(worker),
            t => {
                // If even, the token is a server_to_worker token
                // If odd, it's  a stream token
                if t.0 % 2 == 0 {
                    read_from_server(worker, t);
                } else {
                    read_from_stream(worker, t);
                }
            }
        }
    }
    if readiness.is_writable() {
        let client_id = get_client_from_stream_token(event.token());
        write_to_client(worker, client_id);
    }
}

fn accept_connection(worker: &mut Worker, stream: TcpStream, addr: SocketAddr) {
    let id = Client(worker.client_id_counter);
    worker.client_id_counter += 1;

    let (send1, recv1) = channel();
    let (send2, recv2) = channel();

    let client = ClientHandle {
        stream,
        addr,
        write_buffer: None,
        receiver: recv1,
        sender: send2,
        stream_token: get_stream_token(id),
        server_to_worker_token: get_server_to_worker_token(id),
        manager: ConnectionIOManager::new(),
    };

    worker
        .poll
        .register(
            &client.stream,
            client.stream_token,
            Ready::readable(),
            PollOpt::edge(),
        )
        .unwrap();

    worker
        .poll
        .register(
            &client.receiver,
            client.server_to_worker_token,
            Ready::readable(),
            PollOpt::edge(),
        )
        .unwrap();

    let info = NewClientInfo {
        ip: client.addr.clone(),
        sender: send1,
        receiver: recv2,
    };

    let msg = ListenerToWorkerMessage::NewClient(info);
    worker.sender.send(msg).unwrap();

    worker.clients.insert(id, client);

    trace!("Registered client with ID {}", id.0);
}

fn read_from_listener(worker: &mut Worker) {
    while let Ok(msg) = worker.receiver.try_recv() {
        match msg {
            ListenerToWorkerMessage::ShutDown => worker.running = false,
            ListenerToWorkerMessage::NewConnection(stream, addr) => {
                accept_connection(worker, stream, addr)
            }
            _ => panic!("Invalid message received from listener by worker"),
        }
    }
}

fn read_from_server(worker: &mut Worker, token: Token) {
    let client_id = get_client_from_server_to_worker_token(token);
    let client = worker.clients.get_mut(&client_id).unwrap();

    if let Ok(msg) = client.receiver.try_recv() {
        match msg {
            ServerToWorkerMessage::Disconnect => disconnect_client(worker, client_id),
            ServerToWorkerMessage::SendPacket(packet) => send_packet(worker, client_id, packet),
            ServerToWorkerMessage::EnableCompression(threshold) => client.manager.enable_compression(threshold),
            ServerToWorkerMessage::EnableEncryption(key) => client.manager.enable_encryption(key),
            _ => panic!("Invalid message received from server thread"),
        }
    }
}

fn disconnect_client(worker: &mut Worker, client_id: Client) {
    debug!("Disconnnecting client {}", client_id.0);
    let client = worker.clients.get(&client_id).unwrap();

    worker.poll.deregister(&client.receiver).unwrap();
    worker.poll.deregister(&client.stream).unwrap();

    let _ = client.sender.send(ServerToWorkerMessage::NotifyDisconnect);

    worker.clients.remove(&client_id);
}

fn send_packet(worker: &mut Worker, client_id: Client, packet: Box<Packet>) {
    let client = worker.clients.get_mut(&client_id).unwrap();

    let manager = &mut client.manager;
    let buf = manager.serialize_packet(packet);
    client.write_buffer = Some(buf);

    worker
        .poll
        .reregister(
            &client.stream,
            client.stream_token,
            Ready::readable() | Ready::writable(),
            PollOpt::edge(),
        )
        .unwrap();
}

fn read_from_stream(worker: &mut Worker, token: Token) {
    let client_id = get_client_from_stream_token(token);
    trace!("Reading from stream on client #{}", client_id.0);

    let client = worker.clients.get_mut(&client_id).unwrap();

    let stream = &mut client.stream;

    let mut buf = ByteBuf::with_capacity(128);
    let mut tmp = [0u8; 32];
    while let Ok(amnt) = stream.read(&mut tmp) {
        buf.reserve(amnt);
        buf.put(&mut tmp[0..amnt]);

        if amnt == 0 {
            break;
        }
    }

    if client.manager.accept_data(buf).is_err() {
        disconnect_client(worker, client_id);
    }

    for packet in worker
        .clients
        .get_mut(&client_id)
        .unwrap()
        .manager
        .take_pending_packets()
    {
        handle_packet(worker, client_id, packet);
    }
}

fn write_to_client(worker: &mut Worker, client_id: Client) {
    let client = worker.clients.get_mut(&client_id).unwrap();

    let buf = client.write_buffer.take().unwrap();

    client.stream.write(buf.inner()).unwrap();

    worker
        .poll
        .reregister(
            &client.stream,
            get_stream_token(client_id),
            Ready::readable(),
            PollOpt::edge(),
        )
        .unwrap();
}

fn handle_packet(worker: &mut Worker, client_id: Client, packet: Box<Packet>) {
    trace!("Worker: handle_packet");
    let client = worker.clients.get_mut(&client_id).unwrap();

    let msg = ServerToWorkerMessage::NotifyPacketReceived(packet);
    client.sender.send(msg).unwrap();
}

fn get_stream_token(client_id: Client) -> Token {
    Token(1 + client_id.0 * 2)
}

fn get_server_to_worker_token(client_id: Client) -> Token {
    Token(2 + client_id.0 * 2)
}

fn get_client_from_stream_token(token: Token) -> Client {
    Client((token.0 - 1) / 2)
}

fn get_client_from_server_to_worker_token(token: Token) -> Client {
    trace!("{:?}", token);
    Client(token.0 / 2 - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_stream_token() {
        assert_eq!(get_stream_token(Client(0)), Token(1));
        assert_eq!(get_stream_token(Client(1)), Token(3));
        assert_eq!(get_stream_token(Client(14)), Token(29));
    }

    #[test]
    fn test_get_server_to_worker_token() {
        assert_eq!(get_server_to_worker_token(Client(0)), Token(2));
        assert_eq!(get_server_to_worker_token(Client(1)), Token(4));
        assert_eq!(get_server_to_worker_token(Client(14)), Token(30));
    }

    #[test]
    fn test_get_client_from_stream_token() {
        assert_eq!(get_client_from_stream_token(Token(1)), Client(0));
        assert_eq!(get_client_from_stream_token(Token(3)), Client(1));
        assert_eq!(get_client_from_stream_token(Token(29)), Client(14));
    }

    #[test]
    fn test_get_client_from_server_to_worker_token() {
        assert_eq!(get_client_from_server_to_worker_token(Token(2)), Client(0));
        assert_eq!(get_client_from_server_to_worker_token(Token(4)), Client(1));
        assert_eq!(
            get_client_from_server_to_worker_token(Token(30)),
            Client(14)
        );
    }
}
