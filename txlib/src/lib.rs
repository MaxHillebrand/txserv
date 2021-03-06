// Bitcoin transaction processing & database indexing rust library
// Written in 2020 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

extern crate chrono;
#[macro_use]
extern crate diesel;
#[macro_use]
pub extern crate lnpbp;

pub mod models;
pub mod schema;

pub use lnpbp::common::macros::*;
