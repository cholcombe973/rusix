/*
    helper functions to work with the flatbuffers api
*/
extern crate byteorder;
extern crate flatbuffers;
extern crate nix;
extern crate uint;

use std::os::unix::io::{AsRawFd, RawFd};
use std::path::{Path, PathBuf};

use self::byteorder::{LittleEndian, WriteBytesExt};
use self::flatbuffers::{FlatBufferBuilder, WIPOffset};
use self::nix::{
    errno::Errno as NixErr,
    fcntl::OFlag,
    sys::stat::{FileStat, Mode},
};
use api::service_generated::*;

/*
pub fn create_request(
    parent_inode: u64,
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
*/

// TODO: How are we going to handle rusts ownership system here?  The file descriptor
// needs to stay open and valid for the lifetime of the clients network request.
pub fn create_response(res: (RawFd, FileStat)) -> Vec<u8> {
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
    let stat = filestat_to_iatt(&res.1, &mut builder);
    let preparent = filestat_to_iatt(&res.1, &mut builder);
    let postparent = filestat_to_iatt(&res.1, &mut builder);
    let create = CreateResponse::create(
        &mut builder,
        &CreateResponseArgs {
            result: Some(op_res),
            stat: Some(stat),
            preparent: Some(preparent),
            postparent: Some(postparent),
            fd: res.0 as u64,
        },
    );
    builder.finish_minimal(create);
    builder.finished_data().to_vec()
}

fn errno(i: NixErr) -> Errno {
    match i {
        NixErr::UnknownErrno => Errno::UNKNOWN,
        NixErr::EPERM => Errno::EPERM,
        NixErr::ENOENT => Errno::ENOENT,
        NixErr::ESRCH => Errno::ESRCH,
        NixErr::EINTR => Errno::EINTR,
        NixErr::EIO => Errno::EIO,
        NixErr::ENXIO => Errno::ENXIO,
        NixErr::E2BIG => Errno::E2BIG,
        NixErr::ENOEXEC => Errno::ENOEXEC,
        NixErr::EBADF => Errno::EBADF,
        NixErr::ECHILD => Errno::ECHILD,
        NixErr::EAGAIN => Errno::EAGAIN,
        NixErr::ENOMEM => Errno::ENOMEM,
        NixErr::EACCES => Errno::EACCES,
        NixErr::EFAULT => Errno::EFAULT,
        NixErr::ENOTBLK => Errno::ENOTBLK,
        NixErr::EBUSY => Errno::EBUSY,
        NixErr::EEXIST => Errno::EEXIST,
        NixErr::EXDEV => Errno::EXDEV,
        NixErr::ENODEV => Errno::ENODEV,
        NixErr::ENOTDIR => Errno::ENOTDIR,
        NixErr::EISDIR => Errno::EISDIR,
        NixErr::EINVAL => Errno::EINVAL,
        NixErr::ENFILE => Errno::ENFILE,
        NixErr::EMFILE => Errno::EMFILE,
        NixErr::ENOTTY => Errno::ENOTTY,
        NixErr::ETXTBSY => Errno::ETXTBSY,
        NixErr::EFBIG => Errno::EFBIG,
        NixErr::ENOSPC => Errno::ENOSPC,
        NixErr::ESPIPE => Errno::ESPIPE,
        NixErr::EROFS => Errno::EROFS,
        NixErr::EMLINK => Errno::EMLINK,
        NixErr::EPIPE => Errno::EPIPE,
        NixErr::EDOM => Errno::EDOM,
        NixErr::ERANGE => Errno::ERANGE,
        NixErr::EDEADLK => Errno::EDEADLK,
        NixErr::ENAMETOOLONG => Errno::ENAMETOOLONG,
        NixErr::ENOLCK => Errno::ENOLCK,
        NixErr::ENOSYS => Errno::ENOSYS,
        NixErr::ENOTEMPTY => Errno::ENOTEMPTY,
        NixErr::ELOOP => Errno::ELOOP,
        NixErr::ENOMSG => Errno::ENOMSG,
        NixErr::EIDRM => Errno::EIDRM,
        NixErr::ECHRNG => Errno::ECHRNG,
        NixErr::EL2NSYNC => Errno::EL2NSYNC,
        NixErr::EL3HLT => Errno::EL3HLT,
        NixErr::EL3RST => Errno::EL3RST,
        NixErr::ELNRNG => Errno::ELNRNG,
        NixErr::EUNATCH => Errno::EUNATCH,
        NixErr::ENOCSI => Errno::ENOCSI,
        NixErr::EL2HLT => Errno::EL2HLT,
        NixErr::EBADE => Errno::EBADE,
        NixErr::EBADR => Errno::EBADR,
        NixErr::EXFULL => Errno::EXFULL,
        NixErr::ENOANO => Errno::ENOANO,
        NixErr::EBADRQC => Errno::EBADRQC,
        NixErr::EBADSLT => Errno::EBADSLT,
        NixErr::EBFONT => Errno::EBFONT,
        NixErr::ENOSTR => Errno::ENOSTR,
        NixErr::ENODATA => Errno::ENODATA,
        NixErr::ETIME => Errno::ETIME,
        NixErr::ENOSR => Errno::ENOSR,
        NixErr::ENONET => Errno::ENONET,
        NixErr::ENOPKG => Errno::ENOPKG,
        NixErr::EREMOTE => Errno::EREMOTE,
        NixErr::ENOLINK => Errno::ENOLINK,
        NixErr::EADV => Errno::EADV,
        NixErr::ESRMNT => Errno::ESRMNT,
        NixErr::ECOMM => Errno::ECOMM,
        NixErr::EPROTO => Errno::EPROTO,
        NixErr::EMULTIHOP => Errno::EMULTIHOP,
        NixErr::EDOTDOT => Errno::EDOTDOT,
        NixErr::EBADMSG => Errno::EBADMSG,
        NixErr::EOVERFLOW => Errno::EOVERFLOW,
        NixErr::ENOTUNIQ => Errno::ENOTUNIQ,
        NixErr::EBADFD => Errno::EBADFD,
        NixErr::EREMCHG => Errno::EREMCHG,
        NixErr::ELIBACC => Errno::ELIBACC,
        NixErr::ELIBBAD => Errno::ELIBBAD,
        NixErr::ELIBSCN => Errno::ELIBSCN,
        NixErr::ELIBMAX => Errno::ELIBMAX,
        NixErr::ELIBEXEC => Errno::ELIBEXEC,
        NixErr::EILSEQ => Errno::EILSEQ,
        NixErr::ERESTART => Errno::ERESTART,
        NixErr::ESTRPIPE => Errno::ESTRPIPE,
        NixErr::EUSERS => Errno::EUSERS,
        NixErr::ENOTSOCK => Errno::ENOTSOCK,
        NixErr::EDESTADDRREQ => Errno::EDESTADDRREQ,
        NixErr::EMSGSIZE => Errno::EMSGSIZE,
        NixErr::EPROTOTYPE => Errno::EPROTOTYPE,
        NixErr::ENOPROTOOPT => Errno::ENOPROTOOPT,
        NixErr::EPROTONOSUPPORT => Errno::EPROTONOSUPPORT,
        NixErr::ESOCKTNOSUPPORT => Errno::ESOCKTNOSUPPORT,
        NixErr::EOPNOTSUPP => Errno::EOPNOTSUPP,
        NixErr::EPFNOSUPPORT => Errno::EPFNOSUPPORT,
        NixErr::EAFNOSUPPORT => Errno::EAFNOSUPPORT,
        NixErr::EADDRINUSE => Errno::EADDRINUSE,
        NixErr::EADDRNOTAVAIL => Errno::EADDRNOTAVAIL,
        NixErr::ENETDOWN => Errno::ENETDOWN,
        NixErr::ENETUNREACH => Errno::ENETUNREACH,
        NixErr::ENETRESET => Errno::ENETRESET,
        NixErr::ECONNABORTED => Errno::ECONNABORTED,
        NixErr::ECONNRESET => Errno::ECONNRESET,
        NixErr::ENOBUFS => Errno::ENOBUFS,
        NixErr::EISCONN => Errno::EISCONN,
        NixErr::ENOTCONN => Errno::ENOTCONN,
        NixErr::ESHUTDOWN => Errno::ESHUTDOWN,
        NixErr::ETOOMANYREFS => Errno::ETOOMANYREFS,
        NixErr::ETIMEDOUT => Errno::ETIMEDOUT,
        NixErr::ECONNREFUSED => Errno::ECONNREFUSED,
        NixErr::EHOSTDOWN => Errno::EHOSTDOWN,
        NixErr::EHOSTUNREACH => Errno::EHOSTUNREACH,
        NixErr::EALREADY => Errno::EALREADY,
        NixErr::EINPROGRESS => Errno::EINPROGRESS,
        NixErr::ESTALE => Errno::ESTALE,
        NixErr::EUCLEAN => Errno::EUCLEAN,
        NixErr::ENOTNAM => Errno::ENOTNAM,
        NixErr::ENAVAIL => Errno::ENAVAIL,
        NixErr::EISNAM => Errno::EISNAM,
        NixErr::EREMOTEIO => Errno::EREMOTEIO,
        NixErr::EDQUOT => Errno::EDQUOT,
        NixErr::ENOMEDIUM => Errno::ENOMEDIUM,
        NixErr::EMEDIUMTYPE => Errno::EMEDIUMTYPE,
        NixErr::ECANCELED => Errno::ECANCELED,
        NixErr::ENOKEY => Errno::ENOKEY,
        NixErr::EKEYEXPIRED => Errno::EKEYEXPIRED,
        NixErr::EKEYREVOKED => Errno::EKEYREVOKED,
        NixErr::EKEYREJECTED => Errno::EKEYREJECTED,
        NixErr::EOWNERDEAD => Errno::EOWNERDEAD,
        NixErr::ENOTRECOVERABLE => Errno::ENOTRECOVERABLE,
        _ => Errno::UNKNOWN,
    }
}

// An option failed to unwrap
pub fn op_result<'a>(msg: &str, builder: &mut FlatBufferBuilder<'a>) -> Vec<u8> {
    let error_msg = builder.create_string(msg);
    let errno = errno(NixErr::UnknownErrno);
    let op_res = OpResult::create(
        builder,
        &OpResultArgs {
            result: ResultType::Err,
            errno,
            errorMsg: Some(error_msg),
        },
    );
    builder.finish_minimal(op_res);
    builder.finished_data().to_vec()
}

pub fn err_result(e: nix::Error) -> Vec<u8> {
    let mut builder = flatbuffers::FlatBufferBuilder::new_with_capacity(100);
    let error_msg = builder.create_string(&e.to_string());
    let errno = match e {
        nix::Error::Sys(err) => errno(err),
        nix::Error::InvalidPath => errno(NixErr::UnknownErrno),
        nix::Error::InvalidUtf8 => errno(NixErr::UnknownErrno),
        nix::Error::UnsupportedOperation => errno(NixErr::UnknownErrno),
    };
    let op_res = OpResult::create(
        &mut builder,
        &OpResultArgs {
            result: ResultType::Err,
            errno,
            errorMsg: Some(error_msg),
        },
    );
    builder.finish_minimal(op_res);
    builder.finished_data().to_vec()
}

pub fn filestat_to_iatt<'a>(
    s: &FileStat,
    builder: &mut FlatBufferBuilder<'a>,
) -> WIPOffset<Iatt<'a>> {
    Iatt::create(
        builder,
        &IattArgs {
            //TODO: Do we need the rfid here?
            ia_rfid: None,
            ia_ino: s.st_ino,
            ia_dev: s.st_dev,
            mode: s.st_mode,
            ia_nlink: s.st_nlink,
            ia_uid: s.st_uid,
            ia_gid: s.st_gid,
            ia_rdev: s.st_rdev,
            ia_size: s.st_size,
            ia_blksize: s.st_blksize,
            ia_blocks: s.st_blocks,
            ia_atime: s.st_atime,
            ia_atime_nsec: s.st_atime_nsec,
            ia_mtime: s.st_mtime,
            ia_mtime_nsec: s.st_mtime_nsec,
            ia_ctime: s.st_ctime,
            ia_ctime_nsec: s.st_ctime_nsec,
        },
    )
}

pub fn mkdir_request(parent_inode: u64, mode: Mode, mask: u32, name: &str) -> Vec<u8> {
    let mut builder = flatbuffers::FlatBufferBuilder::new_with_capacity(100);
    let mut tmp = vec![];
    tmp.write_u64::<LittleEndian>(parent_inode).unwrap();
    let hash_data = builder.create_vector(&tmp);
    let rfid = FileHash::create(
        &mut builder,
        &FileHashArgs {
            hash: Some(hash_data),
        },
    );
    let bname = builder.create_string(name);

    let mkdir = MkdirRequest::create(
        &mut builder,
        &MkdirRequestArgs {
            parent_rfid: Some(rfid),
            mode: mode.bits(),
            umask: mask,
            bname: Some(bname),
        },
    );
    let mut args = OperationArgs::default();
    args.mkdir = Some(mkdir);
    let operation = Operation::create(&mut builder, &args);
    builder.finish_minimal(operation);
    builder.finished_data().to_vec()
}

/*
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
*/
/*

*/
pub fn resolve_rfid_to_path(
    rfid: &FileHash,
    backend_path: &Path,
    bname: Option<&str>,
) -> Result<PathBuf, String> {
    let rusix_backend = backend_path.join("/.rusix");
    let hash = rfid
        .hash()
        .ok_or("hash field of FileHash is missing".to_string())?;
    let h = uint::U512::from_little_endian(hash);

    Ok(rusix_backend.join(&format!("{:x}/{:x}/{}", hash[0], hash[1], h.as_usize())))
}

/*
pub fn write_request(inode: &Uuid, flags: OFlag, offset: u64, data: &[u8]) -> Vec<u8> {
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
*/

pub fn write_response(res: (FileStat, FileStat)) -> Vec<u8> {
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
    let pre_stat = filestat_to_iatt(&res.0, &mut builder);
    let post_stat = filestat_to_iatt(&res.1, &mut builder);
    let write = WriteResponse::create(
        &mut builder,
        &WriteResponseArgs {
            result: Some(op_res),
            pre_stat: Some(pre_stat),
            post_stat: Some(post_stat),
        },
    );
    builder.finish_minimal(write);
    builder.finished_data().to_vec()
}

/*
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
*/
