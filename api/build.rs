extern crate capnpc;
extern crate protobuf_codegen_pure;

fn main() {
    protobuf_codegen_pure::run(protobuf_codegen_pure::Args {
        out_dir: "src",
        input: &["protos/err.proto", "protos/service.proto"],
        includes: &["protos"],
        customize: protobuf_codegen_pure::Customize {
            ..Default::default()
        },
    }).expect("protoc");

    ::capnpc::CompilerCommand::new()
        .file("protos/service.capnp")
        .run()
        .expect("compiling schema");
}
