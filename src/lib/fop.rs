/*
    helper functions to work with the flatbuffers api
*/
extern crate flatbuffers;
extern crate uuid;

use self::flatbuffers::{FlatBufferBuilder, WIPOffset};
use self::uuid::Uuid;
use api::service_generated::*;

pub fn stat_request(inode: &Uuid) {
    let mut builder = flatbuffers::FlatBufferBuilder::new_with_capacity(100);
    let rfid = uuid_to_rfid(&inode, &mut builder);

    let stat = StatRequest::create(
        &mut builder,
        &StatRequestArgs {
            rfid: Some(rfid),
            extra_data: None,
        },
    );
    let mut args = OperationArgs::default();
    args.stat = Some(stat);
    let operation = Operation::create(&mut builder, &args);
    builder.finish_minimal(operation);
    let buf = builder.finished_data();
}

fn uuid_to_rfid<'a>(inode: &Uuid, builder: &mut FlatBufferBuilder<'a>) -> WIPOffset<Rfid<'a>> {
    let d4 = builder.create_vector(inode.as_fields().3);

    Rfid::create(
        &mut builder,
        &RfidArgs {
            d1: inode.as_fields().0,
            d2: inode.as_fields().1,
            d3: inode.as_fields().2,
            d4: Some(d4),
        },
    )
}
