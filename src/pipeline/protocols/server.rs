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
use std::time::Instant;

use api::service_generated::*;

use self::flatbuffers::FlatBufferBuilder;
use self::nix::{
    fcntl::{open, OFlag},
    sys::stat::*,
    sys::uio::pwrite,
    unistd::unlink,
};
use self::zmq::{Context, Result as ZmqResult, Socket};

use super::super::super::lib::fop::*;
use super::super::Value;

// Many api responses follow a pattern of
// stat, preparent: Iatt, postparent: Iatt
// so this macro should cover a lot of them
// create needs fd

// link,
// mknod, mkdir,
// rmdir, unlink doesn't have stat
// symlink,
macro_rules! generic_response {
    // With stat
    ($t:ident, $u:ident, $stat:expr, $prestat:expr, $poststat:expr) => {{
        let mut builder = flatbuffers::FlatBufferBuilder::new_with_capacity(100);
        let error_msg = builder.create_string("");
        let op_res = OpResult::create(
            &mut builder,
            &OpResultArgs {
                result: ResultType::Ok,
                errno: Errno::UNKNOWN,
                errorMsg: None,
            },
        );
        let stat = filestat_to_iatt(&$stat, &mut builder);
        let prestat = filestat_to_iatt(&$prestat, &mut builder);
        let poststat = filestat_to_iatt(&$poststat, &mut builder);
        let create = $t::create(
            &mut builder,
            &$u {
                result: Some(op_res),
                stat: Some(stat),
                preparent: Some(prestat),
                postparent: Some(poststat),
            },
        );
        builder.finish_minimal(create);
        builder.finished_data().to_vec()
    }};
    // Without stat
    ($t:ident, $u:ident, $prestat:expr, $poststat:expr) => {{
        let mut builder = flatbuffers::FlatBufferBuilder::new_with_capacity(100);
        let error_msg = builder.create_string("");
        let op_res = OpResult::create(
            &mut builder,
            &OpResultArgs {
                result: ResultType::Ok,
                errno: Errno::UNKNOWN,
                errorMsg: None,
            },
        );
        let prestat = filestat_to_iatt(&$prestat, &mut builder);
        let poststat = filestat_to_iatt(&$poststat, &mut builder);
        let create = $t::create(
            &mut builder,
            &$u {
                result: Some(op_res),
                preparent: Some(prestat),
                postparent: Some(poststat),
            },
        );
        builder.finish_minimal(create);
        builder.finished_data().to_vec()
    }};
}

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

// Server receives an RPC request and responds
impl Server {
    // Start the server
    pub fn new(options: &HashMap<String, Value>, subvolumes: Vec<String>) -> Self {
        Server {}
    }

    pub fn start(&mut self) -> ZmqResult<()> {
        // Preallocate a receiving buffer
        let context = Context::new();
        let frontend = context.socket(zmq::REP).unwrap();
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
        Ok(create_response((fd, stat)))
    }

    fn handle_discard_req<'a>(
        &self,
        w: DiscardRequest,
    ) -> Result<Vec<u8>, Vec<u8>> {
        Ok(vec![])
    }

    fn handle_entrylk_req<'a>(
        &self,
        w: EntrylkRequest,
    ) -> Result<Vec<u8>, Vec<u8>> {
        Ok(vec![])
    }

    fn handle_fallocate_req<'a>(
        &self,
        w: FallocateRequest,
    ) -> Result<Vec<u8>, Vec<u8>> {
        Ok(vec![])
    }

    fn handle_fentrylk_req<'a>(
        &self,
        w: FentrylkRequest,
    ) -> Result<Vec<u8>, Vec<u8>> {
        Ok(vec![])
    }

    fn handle_fgetxattr_req<'a>(
        &self,
        w: FgetxattrRequest,
    ) -> Result<Vec<u8>, Vec<u8>> {
        Ok(vec![])
    }

    fn handle_finodelk_req<'a>(
        &self,
        w: FinodelkRequest,
    ) -> Result<Vec<u8>, Vec<u8>> {
        Ok(vec![])
    }

    fn handle_flush_req<'a>(
        &self,
        w: FlushRequest,
    ) -> Result<Vec<u8>, Vec<u8>> {
        Ok(vec![])
    }

    fn handle_fremovexattr_req<'a>(
        &self,
        w: FremovexattrRequest,
    ) -> Result<Vec<u8>, Vec<u8>> {
        Ok(vec![])
    }

    fn handle_fsetattr_req<'a>(
        &self,
        w: FsetattrRequest,
    ) -> Result<Vec<u8>, Vec<u8>> {
        Ok(vec![])
    }

    fn handle_fsetxattr_req<'a>(
        &self,
        w: FsetxattrRequest,
    ) -> Result<Vec<u8>, Vec<u8>> {
        Ok(vec![])
    }

    fn handle_fstat_req<'a>(
        &self,
        w: FstatRequest,
    ) -> Result<Vec<u8>, Vec<u8>> {
        Ok(vec![])
    }

    fn handle_fsync_req<'a>(
        &self,
        w: FsyncRequest,
    ) -> Result<Vec<u8>, Vec<u8>> {
        Ok(vec![])
    }

    fn handle_fsyncdir_req<'a>(
        &self,
        w: FsyncdirRequest,
    ) -> Result<Vec<u8>, Vec<u8>> {
        Ok(vec![])
    }

    fn handle_ftruncate_req<'a>(
        &self,
        w: FtruncateRequest,
    ) -> Result<Vec<u8>, Vec<u8>> {
        Ok(vec![])
    }

    fn handle_fxattrop_req<'a>(
        &self,
        w: FxattropRequest,
    ) -> Result<Vec<u8>, Vec<u8>> {
        Ok(vec![])
    }

    fn handle_getxattr_req<'a>(
        &self,
        w: GetXattrRequest,
    ) -> Result<Vec<u8>, Vec<u8>> {
        Ok(vec![])
    }

    fn handle_inodelk_req<'a>(
        &self,
        w: InodelkRequest,
    ) -> Result<Vec<u8>, Vec<u8>> {
        Ok(vec![])
    }

    fn handle_ipc_req<'a>(
        &self,
        w: IpcRequest,
    ) -> Result<Vec<u8>, Vec<u8>> {
        Ok(vec![])
    }

    fn handle_lease_req<'a>(
        &self,
        w: LeaseRequest,
    ) -> Result<Vec<u8>, Vec<u8>> {
        Ok(vec![])
    }

    fn handle_link_req<'a>(
        &self,
        w: LinkRequest,
    ) -> Result<Vec<u8>, Vec<u8>> {
        let prestat = fstat(0 as i32).map_err(|e| err_result(e))?;
        let poststat = fstat(0 as i32).map_err(|e| err_result(e))?;
        let stat = fstat(0 as i32).map_err(|e| err_result(e))?;
        Ok(generic_response!(
            LinkResponse,
            LinkResponseArgs,
            prestat,
            stat,
            poststat
        ))
    }

    fn handle_lock_req<'a>(
        &self,
        w: LockRequest,
        builder: &mut FlatBufferBuilder<'a>,
    ) -> Result<Vec<u8>, Vec<u8>> {
        Ok(vec![])
    }

    fn handle_lookup_req<'a>(
        &self,
        w: LookupRequest,
        builder: &mut FlatBufferBuilder<'a>,
    ) -> Result<Vec<u8>, Vec<u8>> {
        Ok(vec![])
    }

    fn handle_mkdir_req<'a>(
        &self,
        w: MkdirRequest,
        builder: &mut FlatBufferBuilder<'a>,
    ) -> Result<Vec<u8>, Vec<u8>> {
        let prestat = fstat(0 as i32).map_err(|e| err_result(e))?;
        // Shouldn't this all be handled elsewhere?  This it the server
        // level and this should've already been figured out by the client
        // Hash the bname // basename
        // Find the parent directory file
        // Modify parent directory file and link in new hash
        // Write new directory file to server where it should belong to
        // Done?
        let poststat = fstat(0 as i32).map_err(|e| err_result(e))?;
        let stat = fstat(0 as i32).map_err(|e| err_result(e))?;
        Ok(generic_response!(
            MkdirResponse,
            MkdirResponseArgs,
            prestat,
            stat,
            poststat
        ))
    }

    fn handle_mknod_req<'a>(
        &self,
        w: MknodRequest,
        builder: &mut FlatBufferBuilder<'a>,
    ) -> Result<Vec<u8>, Vec<u8>> {
        let mode = map_option(Mode::from_bits(w.umask()), op_result, "bad mode", builder)?;
        let kind = map_option(SFlag::from_bits(w.mode()), op_result, "bad kind", builder)?;
        let prestat = fstat(0 as i32).map_err(|e| err_result(e))?;
        mknod(::std::path::Path::new("/"), kind, mode, w.dev()).map_err(|e| err_result(e))?;
        let stat = stat("").map_err(|e| err_result(e))?;
        let poststat = fstat(0 as i32).map_err(|e| err_result(e))?;
        Ok(generic_response!(
            MknodResponse,
            MknodResponseArgs,
            prestat,
            stat,
            poststat
        ))
    }

    fn handle_open_req<'a>(
        &self,
        w: OpenRequest,
        builder: &mut FlatBufferBuilder<'a>,
    ) -> Result<Vec<u8>, Vec<u8>> {
        Ok(vec![])
    }

    fn handle_opendir_req<'a>(
        &self,
        w: OpendirRequest,
        builder: &mut FlatBufferBuilder<'a>,
    ) -> Result<Vec<u8>, Vec<u8>> {
        Ok(vec![])
    }

    fn handle_readdir_req<'a>(
        &self,
        w: ReaddirRequest,
        builder: &mut FlatBufferBuilder<'a>,
    ) -> Result<Vec<u8>, Vec<u8>> {
        Ok(vec![])
    }

    fn handle_readdirp_req<'a>(
        &self,
        w: ReaddirpRequest,
        builder: &mut FlatBufferBuilder<'a>,
    ) -> Result<Vec<u8>, Vec<u8>> {
        Ok(vec![])
    }

    fn handle_readlk_req<'a>(
        &self,
        w: ReadlinkRequest,
        builder: &mut FlatBufferBuilder<'a>,
    ) -> Result<Vec<u8>, Vec<u8>> {
        Ok(vec![])
    }

    fn handle_read_req<'a>(
        &self,
        w: ReadRequest,
        builder: &mut FlatBufferBuilder<'a>,
    ) -> Result<Vec<u8>, Vec<u8>> {
        Ok(vec![])
    }

    fn handle_removexattr_req<'a>(
        &self,
        w: RemovexattrRequest,
        builder: &mut FlatBufferBuilder<'a>,
    ) -> Result<Vec<u8>, Vec<u8>> {
        Ok(vec![])
    }

    fn handle_rename_req<'a>(
        &self,
        w: RenameRequest,
        builder: &mut FlatBufferBuilder<'a>,
    ) -> Result<Vec<u8>, Vec<u8>> {
        Ok(vec![])
    }

    fn handle_rmdir_req<'a>(
        &self,
        w: RmdirRequest,
    ) -> Result<Vec<u8>, Vec<u8>> {
        let prestat = fstat(0 as i32).map_err(|e| err_result(e))?;
        let poststat = fstat(0 as i32).map_err(|e| err_result(e))?;
        Ok(generic_response!(
            RmdirResponse,
            RmdirResponseArgs,
            prestat,
            poststat
        ))
    }

    fn handle_setattr_req<'a>(
        &self,
        w: SetattrRequest,
        builder: &mut FlatBufferBuilder<'a>,
    ) -> Result<Vec<u8>, Vec<u8>> {
        Ok(vec![])
    }

    fn handle_setxattr_req<'a>(
        &self,
        w: SetxattrRequest,
        builder: &mut FlatBufferBuilder<'a>,
    ) -> Result<Vec<u8>, Vec<u8>> {
        Ok(vec![])
    }

    fn handle_stat_req<'a>(
        &self,
        w: StatRequest,
        builder: &mut FlatBufferBuilder<'a>,
    ) -> Result<Vec<u8>, Vec<u8>> {
        Ok(vec![])
    }

    fn handle_statfs_req<'a>(
        &self,
        w: StatfsRequest,
        builder: &mut FlatBufferBuilder<'a>,
    ) -> Result<Vec<u8>, Vec<u8>> {
        Ok(vec![])
    }

    fn handle_symlink_req<'a>(
        &self,
        w: SymlinkRequest,
    ) -> Result<Vec<u8>, Vec<u8>> {
        let prestat = fstat(0 as i32).map_err(|e| err_result(e))?;
        let poststat = fstat(0 as i32).map_err(|e| err_result(e))?;
        let stat = fstat(0 as i32).map_err(|e| err_result(e))?;
        Ok(generic_response!(
            SymlinkResponse,
            SymlinkResponseArgs,
            prestat,
            stat,
            poststat
        ))
    }

    fn handle_truncate_req<'a>(
        &self,
        w: TruncateRequest,
        builder: &mut FlatBufferBuilder<'a>,
    ) -> Result<Vec<u8>, Vec<u8>> {
        //truncate()
        Ok(vec![])
    }

    fn handle_unlink_req<'a>(
        &self,
        w: UnlinkRequest,
    ) -> Result<Vec<u8>, Vec<u8>> {
        let prestat = fstat(0 as i32).map_err(|e| err_result(e))?;
        let poststat = fstat(0 as i32).map_err(|e| err_result(e))?;
        unlink("").map_err(|e| err_result(e))?;

        Ok(generic_response!(
            UnlinkResponse,
            UnlinkResponseArgs,
            prestat,
            poststat
        ))
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
        // The fop_type field is used as a jump table and make identifying
        // which operation the client wants a lot faster.  Otherwise we have
        // to check every field in the table.
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
                self.handle_discard_req(fop.discard().unwrap())?
            }
            Fop::ENTRYLK => {
                let _ = map_option(fop.entrylk(), op_result, "entrylk missing", &mut builder)?;
                self.handle_entrylk_req(fop.entrylk().unwrap())?
            }
            Fop::FALLOCATE => {
                let _ = map_option(
                    fop.fallocate(),
                    op_result,
                    "fallocate missing",
                    &mut builder,
                )?;
                self.handle_fallocate_req(fop.fallocate().unwrap())?
            }
            Fop::FENTRYLK => {
                let _ = map_option(fop.fentrylk(), op_result, "fentrylk missing", &mut builder)?;
                self.handle_fentrylk_req(fop.fentrylk().unwrap())?
            }
            Fop::FGETXATTR => {
                let _ = map_option(
                    fop.fgetxattr(),
                    op_result,
                    "fgetxattr missing",
                    &mut builder,
                )?;
                self.handle_fgetxattr_req(fop.fgetxattr().unwrap())?
            }
            Fop::FINODELK => {
                let _ = map_option(fop.finodelk(), op_result, "finodelk missing", &mut builder)?;
                self.handle_finodelk_req(fop.finodelk().unwrap())?
            }
            Fop::FLUSH => {
                let _ = map_option(fop.flush(), op_result, "flush missing", &mut builder)?;
                self.handle_flush_req(fop.flush().unwrap())?
            }
            Fop::FREMOVEXATTR => {
                let _ = map_option(
                    fop.fremovexattr(),
                    op_result,
                    "fremovexattr missing",
                    &mut builder,
                )?;
                self.handle_fremovexattr_req(fop.fremovexattr().unwrap())?
            }
            Fop::FSETATTR => {
                let _ = map_option(fop.fsetattr(), op_result, "fsetattr missing", &mut builder)?;
                self.handle_fsetattr_req(fop.fsetattr().unwrap())?
            }
            Fop::FSETXATTR => {
                let _ = map_option(
                    fop.fsetxattr(),
                    op_result,
                    "fsetxattr missing",
                    &mut builder,
                )?;
                self.handle_fsetxattr_req(fop.fsetxattr().unwrap())?
            }
            Fop::FSTAT => {
                let _ = map_option(fop.fstat(), op_result, "fstat missing", &mut builder)?;
                self.handle_fstat_req(fop.fstat().unwrap())?
            }
            Fop::FSYNC => {
                let _ = map_option(fop.fsync(), op_result, "fsync missing", &mut builder)?;
                self.handle_fsync_req(fop.fsync().unwrap())?
            }
            Fop::FSYNCDIR => {
                let _ = map_option(fop.fsyncdir(), op_result, "fsyncdir missing", &mut builder)?;
                self.handle_fsyncdir_req(fop.fsyncdir().unwrap())?
            }
            Fop::FTRUNCATE => {
                let _ = map_option(
                    fop.ftruncate(),
                    op_result,
                    "ftruncate missing",
                    &mut builder,
                )?;
                self.handle_ftruncate_req(fop.ftruncate().unwrap())?
            }
            Fop::FXATTROP => {
                let _ = map_option(fop.fxattrop(), op_result, "fxattrop missing", &mut builder)?;
                self.handle_fxattrop_req(fop.fxattrop().unwrap())?
            }
            Fop::GETXATTR => {
                let _ = map_option(fop.getxattr(), op_result, "getxattr missing", &mut builder)?;
                self.handle_getxattr_req(fop.getxattr().unwrap())?
            }
            Fop::INODELK => {
                let _ = map_option(fop.inodelk(), op_result, "inodelk missing", &mut builder)?;
                self.handle_inodelk_req(fop.inodelk().unwrap())?
            }
            Fop::IPC => {
                let _ = map_option(fop.ipc(), op_result, "ipc missing", &mut builder)?;
                self.handle_ipc_req(fop.ipc().unwrap())?
            }
            Fop::LEASE => {
                let _ = map_option(fop.lease(), op_result, "lease missing", &mut builder)?;
                self.handle_lease_req(fop.lease().unwrap())?
            }
            Fop::LINK => {
                let _ = map_option(fop.link(), op_result, "link missing", &mut builder)?;
                self.handle_link_req(fop.link().unwrap())?
            }
            Fop::LK => {
                let _ = map_option(fop.lock(), op_result, "lock missing", &mut builder)?;
                self.handle_lock_req(fop.lock().unwrap(), &mut builder)?
            }
            Fop::LOOKUP => {
                let _ = map_option(fop.lookup(), op_result, "lookup missing", &mut builder)?;
                self.handle_lookup_req(fop.lookup().unwrap(), &mut builder)?
            }
            Fop::MKDIR => {
                let _ = map_option(fop.mkdir(), op_result, "mkdir missing", &mut builder)?;
                self.handle_mkdir_req(fop.mkdir().unwrap(), &mut builder)?
            }
            Fop::MKNOD => {
                let _ = map_option(fop.mknod(), op_result, "mknod missing", &mut builder)?;
                self.handle_mknod_req(fop.mknod().unwrap(), &mut builder)?
            }
            Fop::OPEN => {
                let _ = map_option(fop.open(), op_result, "open missing", &mut builder)?;
                self.handle_open_req(fop.open().unwrap(), &mut builder)?
            }
            Fop::OPENDIR => {
                let _ = map_option(fop.opendir(), op_result, "opendir missing", &mut builder)?;
                self.handle_opendir_req(fop.opendir().unwrap(), &mut builder)?
            }
            Fop::READDIR => {
                let _ = map_option(fop.readdir(), op_result, "readdir missing", &mut builder)?;
                self.handle_readdir_req(fop.readdir().unwrap(), &mut builder)?
            }
            Fop::READDIRP => {
                let _ = map_option(fop.readdirp(), op_result, "readdirp missing", &mut builder)?;
                self.handle_readdirp_req(fop.readdirp().unwrap(), &mut builder)?
            }
            Fop::READLINK => {
                let _ = map_option(fop.readlk(), op_result, "readlink missing", &mut builder)?;
                self.handle_readlk_req(fop.readlk().unwrap(), &mut builder)?
            }
            Fop::READV => {
                let _ = map_option(fop.read(), op_result, "readv missing", &mut builder)?;
                self.handle_read_req(fop.read().unwrap(), &mut builder)?
            }
            Fop::REMOVEXATTR => {
                let _ = map_option(
                    fop.removexattr(),
                    op_result,
                    "removexattr missing",
                    &mut builder,
                )?;
                self.handle_removexattr_req(fop.removexattr().unwrap(), &mut builder)?
            }
            Fop::RENAME => {
                let _ = map_option(fop.rename(), op_result, "rename missing", &mut builder)?;
                self.handle_rename_req(fop.rename().unwrap(), &mut builder)?
            }
            Fop::RMDIR => {
                let _ = map_option(fop.rmdir(), op_result, "rmdir missing", &mut builder)?;
                self.handle_rmdir_req(fop.rmdir().unwrap())?
            }
            Fop::SETATTR => {
                let _ = map_option(fop.setattr(), op_result, "setattr missing", &mut builder)?;
                self.handle_setattr_req(fop.setattr().unwrap(), &mut builder)?
            }
            Fop::SETXATTR => {
                let _ = map_option(fop.setxattr(), op_result, "setxattr missing", &mut builder)?;
                self.handle_setxattr_req(fop.setxattr().unwrap(), &mut builder)?
            }
            Fop::STAT => {
                let _ = map_option(fop.stat(), op_result, "stat missing", &mut builder)?;
                self.handle_stat_req(fop.stat().unwrap(), &mut builder)?
            }
            Fop::STATFS => {
                let _ = map_option(fop.statfs(), op_result, "statfs missing", &mut builder)?;
                self.handle_statfs_req(fop.statfs().unwrap(), &mut builder)?
            }
            Fop::SYMLINK => {
                let _ = map_option(fop.symlink(), op_result, "symlink missing", &mut builder)?;
                self.handle_symlink_req(fop.symlink().unwrap())?
            }
            Fop::TRUNCATE => {
                let _ = map_option(fop.truncate(), op_result, "truncate missing", &mut builder)?;
                self.handle_truncate_req(fop.truncate().unwrap(), &mut builder)?
            }
            Fop::UNLINK => {
                let _ = map_option(fop.unlink(), op_result, "unlink missing", &mut builder)?;
                self.handle_unlink_req(fop.unlink().unwrap())?
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
