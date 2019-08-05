use std::io::Read;
use std::io::Write;
use std::sync::Arc;

use bytes::BufMut;
use hashbrown::HashMap;
use mio::Event;
use mio::{net::TcpStream, Events, Poll, PollOpt, Ready, Token};

use feather_core::bytebuf::ByteBuf;
use feather_core::network::packet::PacketDirection;
use feather_core::network::serialize::ConnectionIOManager;

use crate::config::Config;
use crate::io::initialhandler::{Action, InitialHandler};
use crate::PlayerCount;

use super::*;

// The token used to listen on the channel receiving messages from the listener thread
const LISTENER_TOKEN: Token = Token(0);

struct Worker {
    poll: Poll,
    receiver: Receiver<ListenerToWorkerMessage>,
    sender: Sender<ListenerToWorkerMessage>,
    running: bool,
    client_id_counter: usize,

    clients: HashMap<Client, ClientHandle>,

    pending_disconnects: Vec<Client>,

    config: Arc<Config>,
    player_count: Arc<PlayerCount>,
}

struct ClientHandle {
    stream: TcpStream,
    addr: SocketAddr,

    write_buffer: Option<ByteBuf>,

    receiver: Option<Receiver<ServerToWorkerMessage>>,
    sender: Option<Sender<ServerToWorkerMessage>>,

    stream_token: Token,
    server_to_worker_token: Token,

    manager: ConnectionIOManager,

    initial_handler: Option<InitialHandler>,
}

/// Starts an IO worker on the current thread,
/// blocking indefinitely until a `ShutDown` message
/// is received from the listener.
pub fn start(
    receiver: Receiver<ListenerToWorkerMessage>,
    sender: Sender<ListenerToWorkerMessage>,
    config: Arc<Config>,
    player_count: Arc<PlayerCount>,
) {
    trace!("Starting IO worker thread");
    let poll = Poll::new().unwrap();

    let mut worker = Worker {
        poll,
        receiver,
        sender,
        running: true,

        client_id_counter: 0,
        clients: HashMap::new(),

        pending_disconnects: vec![],
        config,
        player_count,
    };

    worker
        .poll
        .register(
            &worker.receiver,
            LISTENER_TOKEN,
            Ready::readable(),
            PollOpt::edge(),
        )
        .unwrap();

    run_loop(&mut worker);
}

fn run_loop(worker: &mut Worker) {
    let mut events = Events::with_capacity(1024);
    while worker.running {
        worker.poll.poll(&mut events, None).unwrap();

        for event in &events {
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
                } else if read_from_stream(worker, t).is_err() {
                    disconnect_client(worker, get_client_from_stream_token(t));
                }
            }
        }
    }
    if readiness.is_writable() {
        let client_id = get_client_from_stream_token(event.token());
        if write_to_client(worker, client_id).is_err() {
            disconnect_client(worker, client_id);
        }
    }
}

fn accept_connection(worker: &mut Worker, stream: TcpStream, addr: SocketAddr) {
    let id = Client(worker.client_id_counter);
    worker.client_id_counter += 1;

    let client = ClientHandle {
        stream,
        addr,
        write_buffer: None,
        receiver: None,
        sender: None,
        stream_token: get_stream_token(id),
        server_to_worker_token: get_server_to_worker_token(id),
        manager: ConnectionIOManager::new(PacketDirection::Serverbound),
        initial_handler: Some(InitialHandler::new(
            Arc::clone(&worker.config),
            Arc::clone(&worker.player_count),
        )),
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

    while let Ok(msg) = worker
        .clients
        .get_mut(&client_id)
        .unwrap()
        .receiver
        .as_ref()
        .unwrap()
        .try_recv()
    {
        match msg {
            ServerToWorkerMessage::Disconnect => {
                disconnect_client(worker, client_id);
                break;
            }
            ServerToWorkerMessage::SendPacket(packet) => send_packet(worker, client_id, packet),
            _ => panic!("Invalid message received from server thread"),
        }
    }
}

fn disconnect_client(worker: &mut Worker, client_id: Client) {
    let client = worker.clients.get(&client_id).unwrap();

    if client.write_buffer.is_some() {
        // Wait to write data before disconnecting
        worker.pending_disconnects.push(client_id);
        return;
    }

    worker.poll.deregister(&client.stream).unwrap();

    if let Some(sender) = client.sender.as_ref() {
        worker
            .poll
            .deregister(client.receiver.as_ref().unwrap())
            .unwrap();

        let _ = sender.send(ServerToWorkerMessage::NotifyDisconnect);
    }

    debug!("Disconnecting client {}", client_id.0);

    worker.clients.remove(&client_id);
}

fn send_packet(worker: &mut Worker, client_id: Client, packet: Box<dyn Packet>) {
    let client = worker.clients.get_mut(&client_id).unwrap();

    let manager = &mut client.manager;
    let buf = manager.serialize_packet(packet);

    if client.write_buffer.is_some() {
        client
            .write_buffer
            .as_mut()
            .unwrap()
            .write_all(buf.inner())
            .unwrap();
    } else {
        client.write_buffer = Some(buf);
    }

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

fn read_from_stream(worker: &mut Worker, token: Token) -> Result<(), ()> {
    let client_id = get_client_from_stream_token(token);

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

    client.manager.accept_data(buf)?;

    for packet in worker
        .clients
        .get_mut(&client_id)
        .unwrap()
        .manager
        .take_pending_packets()
    {
        handle_packet(worker, client_id, packet);
        if !worker.clients.contains_key(&client_id) {
            // Client was disconnected
            return Ok(());
        }
    }

    Ok(())
}

fn write_to_client(worker: &mut Worker, client_id: Client) -> Result<(), ()> {
    let client = worker.clients.get_mut(&client_id).unwrap();

    let buf = client.write_buffer.take().unwrap();

    client.stream.write(buf.inner()).map_err(|_| ())?;

    worker
        .poll
        .reregister(
            &client.stream,
            get_stream_token(client_id),
            Ready::readable(),
            PollOpt::edge(),
        )
        .unwrap();

    // If client is pending disconnecting, run disconnect() again
    // This is done to allow for data to be written to the client
    // before it is disconnected (otherwise, the client is deregistered
    // from the poller before the data can be written)
    if worker.pending_disconnects.contains(&client_id) {
        disconnect_client(worker, client_id);
    }

    Ok(())
}

fn handle_packet(worker: &mut Worker, client_id: Client, packet: Box<dyn Packet>) {
    let client = worker.clients.get_mut(&client_id).unwrap();

    let mut action_queue = vec![];

    if let Some(ih) = client.initial_handler.as_mut() {
        // Forward packet to the initial handler.
        ih.handle_packet(packet);

        action_queue = ih.actions_to_execute();
    } else {
        // Forward packet to the server.
        let msg = ServerToWorkerMessage::NotifyPacketReceived(packet);
        client.sender.as_ref().unwrap().send(msg).unwrap();
    }

    for action in action_queue {
        let client = worker.clients.get_mut(&client_id).unwrap();

        match action {
            Action::Disconnect => {
                disconnect_client(worker, client_id);
                return;
            }
            Action::EnableCompression(threshold) => {
                client.manager.enable_compression(threshold as usize);
            }
            Action::JoinGame(info) => {
                let (send1, recv1) = channel();
                let (send2, recv2) = channel();

                let player_info = NewClientInfo {
                    ip: client.addr,
                    username: info.username,
                    profile: info.props,
                    uuid: info.uuid,
                    sender: send1,
                    receiver: recv2,
                };

                worker
                    .sender
                    .send(ListenerToWorkerMessage::NewClient(player_info))
                    .unwrap();

                client.initial_handler = None;

                client.sender = Some(send2);
                client.receiver = Some(recv1);

                worker
                    .poll
                    .register(
                        client.receiver.as_ref().unwrap(),
                        client.server_to_worker_token,
                        Ready::readable(),
                        PollOpt::edge(),
                    )
                    .unwrap();
            }
            Action::EnableEncryption(key) => client.manager.enable_encryption(key),
            Action::SendPacket(packet) => send_packet(worker, client_id, packet),
        }
    }
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
