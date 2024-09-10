/*
Copyright (C) 2024  StarlightStargaze

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/
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


