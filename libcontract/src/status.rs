use std::{fs, mem::MaybeUninit, os::fd::AsRawFd};

use crate::libcall;

/// The level of detail obtained when reading the contract.
#[repr(u32)]
pub enum Detail {
    /// CTD_ALL
    All = contract_sys::CTD_ALL,
    /// CTD_COMMON
    Common = contract_sys::CTD_COMMON,
    /// CTD_FIXED
    Fixed = contract_sys::CTD_FIXED,
}

/// Represents a contract's status.
pub struct ContractStatus {
    handle: contract_sys::ct_stathdl_t,
}

impl ContractStatus {
    /// Read the contract for the given contract and detail level.
    pub fn new(ctid: u32, detail: Detail) -> std::io::Result<Self> {
        let file =
            fs::File::open(format!("/system/contract/all/{ctid}/status"))?;
        let mut handle = MaybeUninit::<contract_sys::ct_stathdl_t>::uninit();
        libcall!(ct_status_read(
            file.as_raw_fd(),
            detail as i32,
            handle.as_mut_ptr()
        ))?;

        let handle = unsafe { handle.assume_init() };
        Ok(Self { handle })
    }

    /// Returns a slice of the process IDs of the process contract's members.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use libcontract::status::{ContractStatus, Detail};
    /// let cs = ContractStatus::new(1453, Detail::All).unwrap();
    /// let pids = cs.get_members().unwrap();
    /// println!("The pids in the contract are: {:?}", pids);
    /// ```
    pub fn get_members(&self) -> std::io::Result<&[i32]> {
        let mut numpids = 0;
        let mut pids = MaybeUninit::<*mut libc::pid_t>::uninit();

        let pids = {
            libcall!(ct_pr_status_get_members(
                self.handle,
                pids.as_mut_ptr(),
                &mut numpids,
            ))?;

            unsafe {
                let pids = pids.assume_init();
                if pids.is_null() {
                    &[]
                } else {
                    std::slice::from_raw_parts(pids, numpids as usize)
                }
            }
        };

        Ok(pids)
    }
}

impl Drop for ContractStatus {
    fn drop(&mut self) {
        unsafe {
            contract_sys::ct_status_free(self.handle);
        }
    }
}
