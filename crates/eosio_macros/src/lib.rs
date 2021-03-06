use proc_macro_hack::proc_macro_hack;

pub use eosio_macros_impl::abi;

#[proc_macro_hack(support_nested)]
pub use eosio_macros_impl::print;

#[proc_macro_hack]
pub use eosio_macros_impl::n;

#[proc_macro_hack]
pub use eosio_macros_impl::s;

pub use eosio_macros_impl::{action, name, table, NumBytes, Read, TableRow, Write};
