mod structures;
mod manage_data;
use connect::Connect;
mod linking;
pub mod installation;
mod database;
mod connect;
pub mod removal;

fn main() /*-> Result<(), Error>*/ {
    let mut connect = Connect::new();
    connect.everything();
    for a in connect.vec.0 {
        println!("{a}")
    }
}


