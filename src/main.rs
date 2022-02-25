use serde_json;
use jsonrpc_core::*;
use jsonrpc_core_client::*;

use std::thread;
use std::sync::mpsc::channel;

use std::io::{self, BufRead, BufReader, BufWriter, Error, ErrorKind, Read, Write};
use std::process::{Command, Stdio};

// fn reader_loop (mut reader: BufReader) {
// }

fn main() {
    // let args: String = std::env::args().skip(1);
    let mut child = Command::new("clangd-12")
        // .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        // .stderr(Stdio::piped())
        .spawn()
        .expect("failed to load lsp server");

    let writer = BufWriter::new(child.stdin.take()
        .expect("failed to open stdin lsp server")
    );
    let mut reader = BufReader::new(child.stdout.take()
        .expect("failed to open stdin lsp server")
    );
    // let err = BufReader::new(child.stderr.take()
    //     .expect("failed to open stderr lsp server")
    // );

    // TODO 
    thread::spawn(|| {
    });

    loop {
        let mut buffer = String::new();
        if reader.read_line(&mut buffer).expect("") == 0 {
            println!("not ok. lsp server closed pipe");
            break;
        }
        println!("{:?}", buffer);
    }
}
