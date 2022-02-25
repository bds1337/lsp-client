use serde_json;
use jsonrpc_core::*;
use jsonrpc_core_client::*;

use std::thread;
use std::sync::mpsc::channel;

use std::io::{self, BufRead, BufReader, BufWriter, Error, ErrorKind, Read, Write};
use std::process::{Command, Stdio};
use std::time::Duration;

use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};


// fn reader_loop (mut reader: BufReader) {
// }

fn main() {
    // let args: String = std::env::args().skip(1);
    // let mut child = Command::new("clangd-12")
    let mut child = Command::new("python3")
        .arg("test.py")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to load lsp server");

    let writer = BufWriter::new(child.stdin.take()
        .expect("failed to open stdin lsp server")
    );
    let reader = BufReader::new(child.stdout.take()
        .expect("failed to open stdin lsp server")
    );
    let err = BufReader::new(child.stderr.take()
        .expect("failed to open stderr lsp server")
    );

    loop
    {};

    // TODO 
    // thread::spawn(|| {
    // });

    // let mut buffer = String::new();
    // reader.read_to_string(&mut buffer).expect("err");
    // println!("{}", buffer);

    //FIXME
//     loop {
//         println!("test 1");
//         let mut buffer = String::new();
//         if reader.read_line(&mut buffer).expect("error") == 0 {
//             println!("lsp server closed pipe");
//             break;
//         }
//         println!("{:?}", buffer);

//         println!("test 2");
//         thread::sleep(Duration::from_millis(100));
//         println!("test 3");
//     }
}
