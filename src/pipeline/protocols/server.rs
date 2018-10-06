// Copyright 2018 The Rusix Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate api;
extern crate flatbuffers;
extern crate futures;
extern crate futures_cpupool;
extern crate nix;
extern crate zmq;

use std::collections::HashMap;
use std::io::Cursor;
use std::time::Instant;

use api::service_generated::*;

use self::flatbuffers::{FlatBufferBuilder, WIPOffset};
use self::nix::{
    fcntl::{open, OFlag},
    sys::stat::*,
    sys::uio::pwrite,
};
use self::zmq::{Context, Result as ZmqResult, Socket};

use super::super::super::lib::fop::*;
use super::super::Value;

pub struct Server {
    // Worker pool
// pool: CpuPool,
//frontend: Socket,
//backend: Socket,
}

fn map_option<'a, T, E, F: FnOnce(&str, &mut FlatBufferBuilder) -> E>(
    opt: Option<T>,
    err: F,
    err_msg: &str,
    builder: &mut FlatBufferBuilder<'a>,
) -> Result<T, E> {
    match opt {
        Some(v) => Ok(v),
        None => Err(err(err_msg, builder)),
    }
}

/*
fn map_error<'a, T, E, F, O: FnOnce(nix::Error, &mut FlatBufferBuilder) -> F>(
    res: Result<T, nix::Error>,
    op: O,
    builder: &mut FlatBufferBuilder<'a>,
) -> Result<T, F> {
    match res {
        Ok(t) => Ok(t),
        Err(e) => Err(op(e, builder)),
    }
}
*/

// Server receives an RPC request and responds
impl Server {
    // Start the server
    pub fn new(options: &HashMap<String, Value>, subvolumes: Vec<String>) -> Self {
        Server {}
    }

    pub fn start(&mut self) -> ZmqResult<()> {
        // Preallocate a receiving buffer
        let context = Context::new();
        let mut frontend = context.socket(zmq::REP).unwrap();
        frontend
            .bind("tcp://*:5570")
            .expect("server failed binding frontend");
        loop {
            // Block until we have events to process
            debug!("waiting for packet");
            let msg = frontend.recv_bytes(0)?;
            let response = self.process_fop(&msg);
            match response {
                Ok(o) => frontend.send(&o, 0)?,
                Err(e) => frontend.send(&e, 0)?,
            };
        }
    }

    fn handle_access_req(&self, a: AccessRequest) -> Result<Vec<u8>, Vec<u8>> {
        Ok(vec![])
    }

    fn handle_create_req<'a>(
        &self,
        c: CreateRequest,
        builder: &mut FlatBufferBuilder<'a>,
    ) -> Result<Vec<u8>, Vec<u8>> {
        let flags = map_option(
            OFlag::from_bits(c.flags()),
            op_result,
            "bad oflags",
            builder,
        )?;
        let mode = map_option(Mode::from_bits(c.mode()), op_result, "bad mode", builder)?;
        let fd = open(map_option(c.bname(), op_result, "", builder)?, flags, mode)
            .map_err(|e| err_result(e))?;
        let stat =
            stat(map_option(c.bname(), op_result, "", builder)?).map_err(|e| err_result(e))?;
        Ok(create_response(Ok((fd, stat))))
    }

    fn handle_symlink_req<'a>(
        &self,
        w: SymlinkRequest,
        builder: &mut FlatBufferBuilder<'a>,
    ) -> Result<Vec<u8>, Vec<u8>> {
        Ok(vec![])
    }

    fn handle_truncate_req<'a>(
        &self,
        w: TruncateRequest,
        builder: &mut FlatBufferBuilder<'a>,
    ) -> Result<Vec<u8>, Vec<u8>> {
        Ok(vec![])
    }

    fn handle_unlink_req<'a>(
        &self,
        w: UnlinkRequest,
        builder: &mut FlatBufferBuilder<'a>,
    ) -> Result<Vec<u8>, Vec<u8>> {
        Ok(vec![])
    }

    fn handle_write_req<'a>(
        &self,
        w: WriteRequest,
        builder: &mut FlatBufferBuilder<'a>,
    ) -> Result<Vec<u8>, Vec<u8>> {
        let prestat = fstat(w.fd() as i32).map_err(|e| err_result(e))?;
        let write_len = pwrite(w.fd() as i32, w.data().unwrap(), w.offset() as i64)
            .map_err(|e| err_result(e))?;
        let poststat = fstat(w.fd() as i32).map_err(|e| err_result(e))?;

        Ok(vec![])
    }

    fn handle_xattrop_req<'a>(
        &self,
        w: XattropRequest,
        builder: &mut FlatBufferBuilder<'a>,
    ) -> Result<Vec<u8>, Vec<u8>> {
        Ok(vec![])
    }

    fn handle_zerofill_req<'a>(
        &self,
        w: ZerofillRequest,
        builder: &mut FlatBufferBuilder<'a>,
    ) -> Result<Vec<u8>, Vec<u8>> {
        Ok(vec![])
    }

    // This should process the Fop down to posix and then
    // send the result back to the client.
    fn process_fop(&self, data: &[u8]) -> Result<Vec<u8>, Vec<u8>> {
        let start = Instant::now();
        let fop = get_root_as_operation(&data);
        let elapsed = start.elapsed();
        debug!(
            "flatbuffers loading msg took: {} nanosecs",
            elapsed.subsec_nanos()
        );
        // Figure out what operation is being requested
        // and respond
        let mut builder = flatbuffers::FlatBufferBuilder::new_with_capacity(100);
        let response = match fop.fop_type() {
            Fop::ACCESS => {
                let _ = map_option(
                    fop.access(),
                    op_result,
                    "access field missing",
                    &mut builder,
                )?;
                self.handle_access_req(fop.access().unwrap())?
            }
            Fop::CREATE => {
                let _ = map_option(fop.creat(), op_result, "creat field missing", &mut builder)?;
                self.handle_create_req(fop.creat().unwrap(), &mut builder)?
            }
            Fop::DISCARD => {
                let _ = map_option(fop.discard(), op_result, "discard missing", &mut builder)?;
                vec![]
            }
            Fop::ENTRYLK => {
                let _ = map_option(fop.entrylk(), op_result, "entrylk missing", &mut builder)?;
                vec![]
            }
            Fop::FALLOCATE => {
                let _ = map_option(
                    fop.fallocate(),
                    op_result,
                    "fallocate missing",
                    &mut builder,
                )?;
                vec![]
            }
            Fop::FENTRYLK => {
                let _ = map_option(fop.fentrylk(), op_result, "fentrylk missing", &mut builder)?;
                vec![]
            }
            Fop::FGETXATTR => {
                let _ = map_option(
                    fop.fgetxattr(),
                    op_result,
                    "fgetxattr missing",
                    &mut builder,
                )?;
                vec![]
            }
            Fop::FINODELK => {
                let _ = map_option(fop.finodelk(), op_result, "finodelk missing", &mut builder)?;
                vec![]
            }
            Fop::FLUSH => {
                let _ = map_option(fop.flush(), op_result, "flush missing", &mut builder)?;
                vec![]
            }
            Fop::FREMOVEXATTR => {
                let _ = map_option(
                    fop.fremovexattr(),
                    op_result,
                    "fremovexattr missing",
                    &mut builder,
                )?;
                vec![]
            }
            Fop::FSETATTR => {
                let _ = map_option(fop.fsetattr(), op_result, "fsetattr missing", &mut builder)?;
                vec![]
            }
            Fop::FSETXATTR => {
                let _ = map_option(
                    fop.fsetxattr(),
                    op_result,
                    "fsetxattr missing",
                    &mut builder,
                )?;
                vec![]
            }
            Fop::FSTAT => {
                let _ = map_option(fop.fstat(), op_result, "fstat missing", &mut builder)?;
                vec![]
            }
            Fop::FSYNC => {
                let _ = map_option(fop.fsync(), op_result, "fsync missing", &mut builder)?;
                vec![]
            }
            Fop::FSYNCDIR => {
                let _ = map_option(fop.fsyncdir(), op_result, "fsyncdir missing", &mut builder)?;
                vec![]
            }
            Fop::FTRUNCATE => {
                let _ = map_option(
                    fop.ftruncate(),
                    op_result,
                    "ftruncate missing",
                    &mut builder,
                )?;
                vec![]
            }
            Fop::FXATTROP => {
                let _ = map_option(fop.fxattrop(), op_result, "fxattrop missing", &mut builder)?;
                vec![]
            }
            Fop::GETXATTR => {
                let _ = map_option(fop.getxattr(), op_result, "getxattr missing", &mut builder)?;
                vec![]
            }
            Fop::INODELK => {
                let _ = map_option(fop.inodelk(), op_result, "inodelk missing", &mut builder)?;
                vec![]
            }
            Fop::IPC => {
                let _ = map_option(fop.ipc(), op_result, "ipc missing", &mut builder)?;
                vec![]
            }
            Fop::LEASE => {
                let _ = map_option(fop.lease(), op_result, "lease missing", &mut builder)?;
                vec![]
            }
            Fop::LINK => {
                let _ = map_option(fop.link(), op_result, "link missing", &mut builder)?;
                vec![]
            }
            Fop::LK => {
                let _ = map_option(fop.lock(), op_result, "lock missing", &mut builder)?;
                vec![]
            }
            Fop::LOOKUP => {
                let _ = map_option(fop.lookup(), op_result, "lookup missing", &mut builder)?;
                vec![]
            }
            Fop::MKDIR => {
                let _ = map_option(fop.mkdir(), op_result, "mkdir missing", &mut builder)?;
                vec![]
            }
            Fop::MKNOD => {
                let _ = map_option(fop.mknod(), op_result, "mknod missing", &mut builder)?;
                vec![]
            }
            Fop::OPEN => {
                let _ = map_option(fop.open(), op_result, "open missing", &mut builder)?;
                vec![]
            }
            Fop::OPENDIR => {
                let _ = map_option(fop.opendir(), op_result, "opendir missing", &mut builder)?;
                vec![]
            }
            Fop::READDIR => {
                let _ = map_option(fop.readdir(), op_result, "readdir missing", &mut builder)?;
                vec![]
            }
            Fop::READDIRP => {
                let _ = map_option(fop.readdirp(), op_result, "readdirp missing", &mut builder)?;
                vec![]
            }
            Fop::READLINK => {
                let _ = map_option(fop.readlk(), op_result, "readlink missing", &mut builder)?;
                vec![]
            }
            Fop::READV => {
                let _ = map_option(fop.read(), op_result, "readv missing", &mut builder)?;
                vec![]
            }
            Fop::REMOVEXATTR => {
                let _ = map_option(
                    fop.removexattr(),
                    op_result,
                    "removexattr missing",
                    &mut builder,
                )?;
                vec![]
            }
            Fop::RENAME => {
                let _ = map_option(fop.rename(), op_result, "rename missing", &mut builder)?;
                vec![]
            }
            Fop::RMDIR => {
                let _ = map_option(fop.rmdir(), op_result, "rmdir missing", &mut builder)?;
                vec![]
            }
            Fop::SETATTR => {
                let _ = map_option(fop.setattr(), op_result, "setattr missing", &mut builder)?;
                vec![]
            }
            Fop::SETXATTR => {
                let _ = map_option(fop.setxattr(), op_result, "setxattr missing", &mut builder)?;
                vec![]
            }
            Fop::STAT => {
                let _ = map_option(fop.stat(), op_result, "stat missing", &mut builder)?;
                vec![]
            }
            Fop::STATFS => {
                let _ = map_option(fop.statfs(), op_result, "statfs missing", &mut builder)?;
                vec![]
            }
            Fop::SYMLINK => {
                let _ = map_option(fop.symlink(), op_result, "symlink missing", &mut builder)?;
                self.handle_symlink_req(fop.symlink().unwrap(), &mut builder)?
            }
            Fop::TRUNCATE => {
                let _ = map_option(fop.truncate(), op_result, "truncate missing", &mut builder)?;
                self.handle_truncate_req(fop.truncate().unwrap(), &mut builder)?
            }
            Fop::UNLINK => {
                let _ = map_option(fop.unlink(), op_result, "unlink missing", &mut builder)?;
                self.handle_unlink_req(fop.unlink().unwrap(), &mut builder)?
            }
            Fop::WRITEV => {
                let _ = map_option(fop.write(), op_result, "writev missing", &mut builder)?;
                self.handle_write_req(fop.write().unwrap(), &mut builder)?
            }
            Fop::XATTROP => {
                let _ = map_option(fop.xattrop(), op_result, "xattrop missing", &mut builder)?;
                self.handle_xattrop_req(fop.xattrop().unwrap(), &mut builder)?
            }
            Fop::ZEROFILL => {
                let _ = map_option(fop.zerofill(), op_result, "zerofill missing", &mut builder)?;
                self.handle_zerofill_req(fop.zerofill().unwrap(), &mut builder)?
            }
        };
        debug!(
            "Operation took: {} nanosecs",
            start.elapsed().subsec_nanos()
        );
        Ok(response)
    }

    pub fn stop(&self) {}
}
