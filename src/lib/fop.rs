/*
    helper functions to work with the flatbuffers api
*/
extern crate flatbuffers;
extern crate nix;
extern crate uuid;

use std::fs::{File, Metadata};
use std::io::Result as IOResult;
use std::os::unix::io::AsRawFd;

use self::flatbuffers::{FlatBufferBuilder, WIPOffset};
use self::nix::{fcntl::OFlag, sys::stat::Mode};
use self::uuid::Uuid;
use api::service_generated::*;

pub fn create_request(
    parent_inode: &Uuid,
    flags: OFlag,
    mode: Mode,
    umask: Mode,
    name: &str,
) -> Vec<u8> {
    let mut builder = flatbuffers::FlatBufferBuilder::new_with_capacity(100);
    let rfid = uuid_to_rfid(&parent_inode, &mut builder);
    let bname = builder.create_string(name);

    let create = CreateRequest::create(
        &mut builder,
        &CreateRequestArgs {
            parent_rfid: Some(rfid),
            flags: flags.bits(),
            mode: mode.bits(),
            umask: umask.bits(),
            bname: Some(bname),
        },
    );
    let mut args = OperationArgs::default();
    args.creat = Some(create);
    let operation = Operation::create(&mut builder, &args);
    builder.finish_minimal(operation);
    builder.finished_data().to_vec()
}

// Response's all include a OpResult field which says if they succeeded or failed
// and why
// TODO: How are we going to handle rusts ownership system here?  The file descriptor
// needs to stay open and valid for the lifetime of the clients network request.
pub fn create_response(result: IOResult<(File, Metadata)>) -> Vec<u8> {
    match result {
        Ok(res) => {
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
            let stat = Iatt::create(&mut builder, &IattArgs{
                ia_rfid: None,
                ia_ino: 0,
                ia_dev:0,
                mode:0,
                ia_nlink:0,
                ia_uid:0,
                ia_gid:0,
                ia_rdev:0,
                ia_size:0,
                ia_blksize:0,
                ia_blocks:0,
                ia_atime:0,
                ia_atime_nsec:0,
                ia_mtime:0,
                ia_mtime_nsec:0,
                ia_ctime:0,
                ia_ctime_nsec:0,
            });
            let create = CreateResponse::create(
                &mut builder,
                &CreateResponseArgs {
                    result: Some(op_res),
                    stat: Some(stat),
                    fd: res.0.as_raw_fd() as u64,
                },
            );
            builder.finish_minimal(create);
            builder.finished_data().to_vec()
        }
        Err(e) => {
            let mut builder = flatbuffers::FlatBufferBuilder::new_with_capacity(100);
            let error_msg = builder.create_string(&e.to_string());
            let op_res = OpResult::create(
                &mut builder,
                &OpResultArgs {
                    result: ResultType::Err,
                    errno: Errno::UNKNOWN,
                    errorMsg: None,
                },
            );
            let create = CreateResponse::create(
                &mut builder,
                &CreateResponseArgs {
                    result: Some(op_res),
                    stat: None,
                    fd: 0,
                },
            );
            builder.finish_minimal(create);
            builder.finished_data().to_vec()
        }
    }
}

pub fn stat_request(inode: &Uuid) -> Vec<u8> {
    let mut builder = flatbuffers::FlatBufferBuilder::new_with_capacity(100);
    let rfid = uuid_to_rfid(&inode, &mut builder);

    let stat = StatRequest::create(&mut builder, &StatRequestArgs { rfid: Some(rfid) });
    let mut args = OperationArgs::default();
    args.stat = Some(stat);
    let operation = Operation::create(&mut builder, &args);
    builder.finish_minimal(operation);
    builder.finished_data().to_vec()
}

pub fn read_request<'a>(inode: &Uuid, flags: OFlag, offset: u64, size: u64) -> Vec<u8> {
    let mut builder = flatbuffers::FlatBufferBuilder::new_with_capacity(100);
    let rfid = uuid_to_rfid(&inode, &mut builder);

    let read = ReadRequest::create(
        &mut builder,
        &ReadRequestArgs {
            rfid: Some(rfid),
            size_: size,
            offset: offset,
            flag: flags.bits(),
            fd: 0,
        },
    );
    let mut args = OperationArgs::default();
    args.read = Some(read);
    let operation = Operation::create(&mut builder, &args);
    builder.finish_minimal(operation);
    builder.finished_data().to_vec()
}

pub fn write_request<'a>(inode: &Uuid, flags: OFlag, offset: u64, data: &[u8]) -> Vec<u8> {
    let mut builder = flatbuffers::FlatBufferBuilder::new_with_capacity(100);
    let data = builder.create_vector(data);
    let rfid = uuid_to_rfid(&inode, &mut builder);

    let write = WriteRequest::create(
        &mut builder,
        &WriteRequestArgs {
            rfid: Some(rfid),
            data: Some(data),
            offset: offset,
            flag: flags.bits(),
            fd: 0,
        },
    );
    let mut args = OperationArgs::default();
    args.write = Some(write);
    let operation = Operation::create(&mut builder, &args);
    builder.finish_minimal(operation);
    builder.finished_data().to_vec()
}

fn uuid_to_rfid<'a>(inode: &Uuid, builder: &mut FlatBufferBuilder<'a>) -> WIPOffset<Rfid<'a>> {
    let d4 = builder.create_vector(inode.as_fields().3);

    Rfid::create(
        builder,
        &RfidArgs {
            d1: inode.as_fields().0,
            d2: inode.as_fields().1,
            d3: inode.as_fields().2,
            d4: Some(d4),
        },
    )
}
