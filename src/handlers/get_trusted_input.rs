use crate::AppSW;
use ledger_device_sdk::io::Comm;

pub fn handler_get_trusted_input(comm: &mut Comm, _first: bool, _next: bool) -> Result<(), AppSW> {
    // Try to get data from comm
    let _data = comm.get_data().map_err(|_| AppSW::WrongApduLength)?;

    Err(AppSW::InsNotSupported)
}
