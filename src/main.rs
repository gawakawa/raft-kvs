use std::{collections::HashMap, net::SocketAddr};

use raftpico::{FileStorage, Machine, Server};
use serde::{Deserialize, Serialize};

struct KvsMachine {
    entries: HashMap<String, serde_json::Value>,
}

impl Machine for KvsMachine {}

#[derive(serde::Serialize, serde::Deserialize)]
enum KvsInput {
    Put {
        key: String,
        value: serde_json::Value,
    },
    Get {
        key: String,
    },
    Delete {
        key: String,
    },
}

type Input = KvsInput;

fn apply(&mut self, ctx: &mut ApplyContext, input: Self::Input) {
    match input {
        KvsInput::Put { key, value } => {
            let old_value = self.entries.insert(key, value);
            ctx.output(&old_value);
        }
        KvsInput::Get { key } => {
            let value = self.entries.get(&key);
            ctx.output(&value);
        }
        KvsInput::Delete { key } => {
            let value = self.entries.remove(&key);
            ctx.output(&value);
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listen_addr: SocketAddr = format!("IPアドレス:TCPポート").parse()?;
    let storage = Some(FileStorage::new(format!("ファイルパス"))?);

    let mut server = Server::<T>::start(listen_addr, storage)?;

    loop {
        server.poll(None)?;
    }
}
