use ledger_device_sdk::io::Comm;

use crate::{handlers::sign_tx::TxContext, AppSW};

pub fn handler_sign_msg(
    comm: &mut Comm,
    _ctx: &mut TxContext,
    _first: bool,
    _next: bool,
) -> Result<(), AppSW> {
    // Try to get data from comm
    let _data = comm.get_data().map_err(|_| AppSW::WrongApduLength)?;

    Err(AppSW::InsNotSupported)
}
