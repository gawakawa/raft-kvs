use std::{collections::HashMap, net::SocketAddr};

use raftpico::{ApplyContext, FileStorage, Machine, Server};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port: u16 = std::env::args()
        .nth(1) // プログラムの引数としてポート番号を受け取る
        .and_then(|a| a.parse().ok())
        .expect("invalid command line arg");

    let listen_addr: SocketAddr = format!("127.0.0.1:{port}").parse()?;

    // "raftkvs-{port}.jsonl" に Raft ノードの状態やログを記録
    let storage = Some(FileStorage::new(format!("raftkvs-{port}.jsonl"))?);

    // Raft ノードとして動作するサーバーを構築
    let mut server = Server::<KvsMachine>::start(listen_addr, storage)?;

    // サーバーの処理を実行する
    loop {
        server.poll(None)?;
    }
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
struct KvsMachine {
    entries: HashMap<String, serde_json::Value>,
}

impl Machine for KvsMachine {
    type Input = KvsInput;

    // KVS サーバーの動作
    fn apply(&mut self, ctx: &mut ApplyContext, input: Self::Input) {
        match input {
            // ハッシュマップにキーと値の組を登録する
            KvsInput::Put { key, value } => {
                let old_value = self.entries.insert(key, value);
                ctx.output(&old_value);
            }

            // ハッシュマップからキーに対応する値を取り出す
            KvsInput::Get { key } => {
                let value = self.entries.get(&key);
                ctx.output(&value);
            }

            // ハッシュマップからキーとその値を削除する
            KvsInput::Delete { key } => {
                let value = self.entries.remove(&key);
                ctx.output(&value);
            }
        }
    }
}

// KVS サーバーへのリクエストのデータ構造
#[derive(Debug, serde::Serialize, serde::Deserialize)]
enum KvsInput {
    // ハッシュマップにキーと値の組を登録する
    Put {
        key: String,
        value: serde_json::Value,
    },

    // ハッシュマップからキーに対応する値を取り出す
    Get {
        key: String,
    },

    // ハッシュマップからキーとその値を削除する
    Delete {
        key: String,
    },
}
