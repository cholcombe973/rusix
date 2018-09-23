/*
    helper functions to work with the flatbuffers api
*/
extern crate flatbuffers;
extern crate nix;
extern crate uuid;

use self::flatbuffers::{FlatBufferBuilder, WIPOffset};
use self::nix::fcntl::OFlag;
use self::nix::sys::stat::Mode;
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
