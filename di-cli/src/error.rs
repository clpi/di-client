// use anyhow::{}
//

use std::{fmt, io, error};

#[derive(Debug)]
pub(crate) struct CmdError {
    kind: _CmdErrorKind,
}

#[derive(Debug)]
pub(crate) enum _CmdErrorKind {
    _Io(io::Error),
    _NotFound(Vec<u8>),
    _StdErr(Vec<u8>),
}

impl CmdError {

    pub fn _io(err: io::Error) -> CmdError {
        CmdError { kind: _CmdErrorKind::_Io(err) }
    }

    pub fn _stderr(err: Vec<u8>) -> CmdError {
        CmdError { kind: _CmdErrorKind::_StdErr(err) }
    }
}

impl error::Error for CmdError {
    fn description(&self) -> &str { "err" }
}

impl fmt::Display for CmdError {

    fn fmt(&self, _formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}
