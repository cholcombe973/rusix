// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum RusixErrno {
    EPERM = 1,
    ENOENT = 2,
    ESRCH = 3,
    EINTR = 4,
    EIO = 5,
    ENXIO = 6,
    E2BIG = 7,
    ENOEXEC = 8,
    EBADF = 9,
    ECHILD = 10,
    EAGAIN = 11,
    ENOMEM = 12,
    EACCES = 13,
    EFAULT = 14,
    ENOTBLK = 15,
    EBUSY = 16,
    EEXIST = 17,
    EXDEV = 18,
    ENODEV = 19,
    ENOTDIR = 20,
    EISDIR = 21,
    EINVAL = 22,
    ENFILE = 23,
    EMFILE = 24,
    ENOTTY = 25,
    ETXTBSY = 26,
    EFBIG = 27,
    ENOSPC = 28,
    ESPIPE = 29,
    EROFS = 30,
    EMLINK = 31,
    EPIPE = 32,
    EDOM = 33,
    ERANGE = 34,
    EDEADLK = 35,
    ENAMETOOLONG = 36,
    ENOLCK = 37,
    ENOSYS = 38,
    ENOTEMPTY = 39,
    ELOOP = 40,
    ENOMSG = 42,
    EIDRM = 43,
    ECHRNG = 44,
    EL2NSYNC = 45,
    EL3HLT = 46,
    EL3RST = 47,
    ELNRNG = 48,
    EUNATCH = 49,
    ENOCSI = 50,
    EL2HLT = 51,
    EBADE = 52,
    EBADR = 53,
    EXFULL = 54,
    ENOANO = 55,
    EBADRQC = 56,
    EBADSLT = 57,
    EBFONT = 59,
    ENOSTR = 60,
    ENODATA = 61,
    ETIME = 62,
    ENOSR = 63,
    ENONET = 64,
    ENOPKG = 65,
    EREMOTE = 66,
    ENOLINK = 67,
    EADV = 68,
    ESRMNT = 69,
    ECOMM = 70,
    EPROTO = 71,
    EMULTIHOP = 72,
    EDOTDOT = 73,
    EBADMSG = 74,
    EOVERFLOW = 75,
    ENOTUNIQ = 76,
    EBADFD = 77,
    EREMCHG = 78,
    ELIBACC = 79,
    ELIBBAD = 80,
    ELIBSCN = 81,
    ELIBMAX = 82,
    ELIBEXEC = 83,
    EILSEQ = 84,
    ERESTART = 85,
    ESTRPIPE = 86,
    EUSERS = 87,
    ENOTSOCK = 88,
    EDESTADDRREQ = 89,
    EMSGSIZE = 90,
    EPROTOTYPE = 91,
    ENOPROTOOPT = 92,
    EPROTONOSUPPORT = 93,
    ESOCKTNOSUPPORT = 94,
    EOPNOTSUPP = 95,
    EPFNOSUPPORT = 96,
    EAFNOSUPPORT = 97,
    EADDRINUSE = 98,
    EADDRNOTAVAIL = 99,
    ENETDOWN = 100,
    ENETUNREACH = 101,
    ENETRESET = 102,
    ECONNABORTED = 103,
    ECONNRESET = 104,
    ENOBUFS = 105,
    EISCONN = 106,
    ENOTCONN = 107,
    ESHUTDOWN = 108,
    ETOOMANYREFS = 109,
    ETIMEDOUT = 110,
    ECONNREFUSED = 111,
    EHOSTDOWN = 112,
    EHOSTUNREACH = 113,
    EALREADY = 114,
    EINPROGRESS = 115,
    ESTALE = 116,
    EUCLEAN = 117,
    ENOTNAM = 118,
    ENAVAIL = 119,
    EISNAM = 120,
    EREMOTEIO = 121,
    EDQUOT = 122,
    ENOMEDIUM = 123,
    EMEDIUMTYPE = 124,
    ECANCELED = 125,
    ENOKEY = 126,
    EKEYEXPIRED = 127,
    EKEYREVOKED = 128,
    EKEYREJECTED = 129,
    EOWNERDEAD = 130,
    ENOTRECOVERABLE = 131,
}

impl ::protobuf::ProtobufEnum for RusixErrno {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<RusixErrno> {
        match value {
            1 => ::std::option::Option::Some(RusixErrno::EPERM),
            2 => ::std::option::Option::Some(RusixErrno::ENOENT),
            3 => ::std::option::Option::Some(RusixErrno::ESRCH),
            4 => ::std::option::Option::Some(RusixErrno::EINTR),
            5 => ::std::option::Option::Some(RusixErrno::EIO),
            6 => ::std::option::Option::Some(RusixErrno::ENXIO),
            7 => ::std::option::Option::Some(RusixErrno::E2BIG),
            8 => ::std::option::Option::Some(RusixErrno::ENOEXEC),
            9 => ::std::option::Option::Some(RusixErrno::EBADF),
            10 => ::std::option::Option::Some(RusixErrno::ECHILD),
            11 => ::std::option::Option::Some(RusixErrno::EAGAIN),
            12 => ::std::option::Option::Some(RusixErrno::ENOMEM),
            13 => ::std::option::Option::Some(RusixErrno::EACCES),
            14 => ::std::option::Option::Some(RusixErrno::EFAULT),
            15 => ::std::option::Option::Some(RusixErrno::ENOTBLK),
            16 => ::std::option::Option::Some(RusixErrno::EBUSY),
            17 => ::std::option::Option::Some(RusixErrno::EEXIST),
            18 => ::std::option::Option::Some(RusixErrno::EXDEV),
            19 => ::std::option::Option::Some(RusixErrno::ENODEV),
            20 => ::std::option::Option::Some(RusixErrno::ENOTDIR),
            21 => ::std::option::Option::Some(RusixErrno::EISDIR),
            22 => ::std::option::Option::Some(RusixErrno::EINVAL),
            23 => ::std::option::Option::Some(RusixErrno::ENFILE),
            24 => ::std::option::Option::Some(RusixErrno::EMFILE),
            25 => ::std::option::Option::Some(RusixErrno::ENOTTY),
            26 => ::std::option::Option::Some(RusixErrno::ETXTBSY),
            27 => ::std::option::Option::Some(RusixErrno::EFBIG),
            28 => ::std::option::Option::Some(RusixErrno::ENOSPC),
            29 => ::std::option::Option::Some(RusixErrno::ESPIPE),
            30 => ::std::option::Option::Some(RusixErrno::EROFS),
            31 => ::std::option::Option::Some(RusixErrno::EMLINK),
            32 => ::std::option::Option::Some(RusixErrno::EPIPE),
            33 => ::std::option::Option::Some(RusixErrno::EDOM),
            34 => ::std::option::Option::Some(RusixErrno::ERANGE),
            35 => ::std::option::Option::Some(RusixErrno::EDEADLK),
            36 => ::std::option::Option::Some(RusixErrno::ENAMETOOLONG),
            37 => ::std::option::Option::Some(RusixErrno::ENOLCK),
            38 => ::std::option::Option::Some(RusixErrno::ENOSYS),
            39 => ::std::option::Option::Some(RusixErrno::ENOTEMPTY),
            40 => ::std::option::Option::Some(RusixErrno::ELOOP),
            42 => ::std::option::Option::Some(RusixErrno::ENOMSG),
            43 => ::std::option::Option::Some(RusixErrno::EIDRM),
            44 => ::std::option::Option::Some(RusixErrno::ECHRNG),
            45 => ::std::option::Option::Some(RusixErrno::EL2NSYNC),
            46 => ::std::option::Option::Some(RusixErrno::EL3HLT),
            47 => ::std::option::Option::Some(RusixErrno::EL3RST),
            48 => ::std::option::Option::Some(RusixErrno::ELNRNG),
            49 => ::std::option::Option::Some(RusixErrno::EUNATCH),
            50 => ::std::option::Option::Some(RusixErrno::ENOCSI),
            51 => ::std::option::Option::Some(RusixErrno::EL2HLT),
            52 => ::std::option::Option::Some(RusixErrno::EBADE),
            53 => ::std::option::Option::Some(RusixErrno::EBADR),
            54 => ::std::option::Option::Some(RusixErrno::EXFULL),
            55 => ::std::option::Option::Some(RusixErrno::ENOANO),
            56 => ::std::option::Option::Some(RusixErrno::EBADRQC),
            57 => ::std::option::Option::Some(RusixErrno::EBADSLT),
            59 => ::std::option::Option::Some(RusixErrno::EBFONT),
            60 => ::std::option::Option::Some(RusixErrno::ENOSTR),
            61 => ::std::option::Option::Some(RusixErrno::ENODATA),
            62 => ::std::option::Option::Some(RusixErrno::ETIME),
            63 => ::std::option::Option::Some(RusixErrno::ENOSR),
            64 => ::std::option::Option::Some(RusixErrno::ENONET),
            65 => ::std::option::Option::Some(RusixErrno::ENOPKG),
            66 => ::std::option::Option::Some(RusixErrno::EREMOTE),
            67 => ::std::option::Option::Some(RusixErrno::ENOLINK),
            68 => ::std::option::Option::Some(RusixErrno::EADV),
            69 => ::std::option::Option::Some(RusixErrno::ESRMNT),
            70 => ::std::option::Option::Some(RusixErrno::ECOMM),
            71 => ::std::option::Option::Some(RusixErrno::EPROTO),
            72 => ::std::option::Option::Some(RusixErrno::EMULTIHOP),
            73 => ::std::option::Option::Some(RusixErrno::EDOTDOT),
            74 => ::std::option::Option::Some(RusixErrno::EBADMSG),
            75 => ::std::option::Option::Some(RusixErrno::EOVERFLOW),
            76 => ::std::option::Option::Some(RusixErrno::ENOTUNIQ),
            77 => ::std::option::Option::Some(RusixErrno::EBADFD),
            78 => ::std::option::Option::Some(RusixErrno::EREMCHG),
            79 => ::std::option::Option::Some(RusixErrno::ELIBACC),
            80 => ::std::option::Option::Some(RusixErrno::ELIBBAD),
            81 => ::std::option::Option::Some(RusixErrno::ELIBSCN),
            82 => ::std::option::Option::Some(RusixErrno::ELIBMAX),
            83 => ::std::option::Option::Some(RusixErrno::ELIBEXEC),
            84 => ::std::option::Option::Some(RusixErrno::EILSEQ),
            85 => ::std::option::Option::Some(RusixErrno::ERESTART),
            86 => ::std::option::Option::Some(RusixErrno::ESTRPIPE),
            87 => ::std::option::Option::Some(RusixErrno::EUSERS),
            88 => ::std::option::Option::Some(RusixErrno::ENOTSOCK),
            89 => ::std::option::Option::Some(RusixErrno::EDESTADDRREQ),
            90 => ::std::option::Option::Some(RusixErrno::EMSGSIZE),
            91 => ::std::option::Option::Some(RusixErrno::EPROTOTYPE),
            92 => ::std::option::Option::Some(RusixErrno::ENOPROTOOPT),
            93 => ::std::option::Option::Some(RusixErrno::EPROTONOSUPPORT),
            94 => ::std::option::Option::Some(RusixErrno::ESOCKTNOSUPPORT),
            95 => ::std::option::Option::Some(RusixErrno::EOPNOTSUPP),
            96 => ::std::option::Option::Some(RusixErrno::EPFNOSUPPORT),
            97 => ::std::option::Option::Some(RusixErrno::EAFNOSUPPORT),
            98 => ::std::option::Option::Some(RusixErrno::EADDRINUSE),
            99 => ::std::option::Option::Some(RusixErrno::EADDRNOTAVAIL),
            100 => ::std::option::Option::Some(RusixErrno::ENETDOWN),
            101 => ::std::option::Option::Some(RusixErrno::ENETUNREACH),
            102 => ::std::option::Option::Some(RusixErrno::ENETRESET),
            103 => ::std::option::Option::Some(RusixErrno::ECONNABORTED),
            104 => ::std::option::Option::Some(RusixErrno::ECONNRESET),
            105 => ::std::option::Option::Some(RusixErrno::ENOBUFS),
            106 => ::std::option::Option::Some(RusixErrno::EISCONN),
            107 => ::std::option::Option::Some(RusixErrno::ENOTCONN),
            108 => ::std::option::Option::Some(RusixErrno::ESHUTDOWN),
            109 => ::std::option::Option::Some(RusixErrno::ETOOMANYREFS),
            110 => ::std::option::Option::Some(RusixErrno::ETIMEDOUT),
            111 => ::std::option::Option::Some(RusixErrno::ECONNREFUSED),
            112 => ::std::option::Option::Some(RusixErrno::EHOSTDOWN),
            113 => ::std::option::Option::Some(RusixErrno::EHOSTUNREACH),
            114 => ::std::option::Option::Some(RusixErrno::EALREADY),
            115 => ::std::option::Option::Some(RusixErrno::EINPROGRESS),
            116 => ::std::option::Option::Some(RusixErrno::ESTALE),
            117 => ::std::option::Option::Some(RusixErrno::EUCLEAN),
            118 => ::std::option::Option::Some(RusixErrno::ENOTNAM),
            119 => ::std::option::Option::Some(RusixErrno::ENAVAIL),
            120 => ::std::option::Option::Some(RusixErrno::EISNAM),
            121 => ::std::option::Option::Some(RusixErrno::EREMOTEIO),
            122 => ::std::option::Option::Some(RusixErrno::EDQUOT),
            123 => ::std::option::Option::Some(RusixErrno::ENOMEDIUM),
            124 => ::std::option::Option::Some(RusixErrno::EMEDIUMTYPE),
            125 => ::std::option::Option::Some(RusixErrno::ECANCELED),
            126 => ::std::option::Option::Some(RusixErrno::ENOKEY),
            127 => ::std::option::Option::Some(RusixErrno::EKEYEXPIRED),
            128 => ::std::option::Option::Some(RusixErrno::EKEYREVOKED),
            129 => ::std::option::Option::Some(RusixErrno::EKEYREJECTED),
            130 => ::std::option::Option::Some(RusixErrno::EOWNERDEAD),
            131 => ::std::option::Option::Some(RusixErrno::ENOTRECOVERABLE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [RusixErrno] = &[
            RusixErrno::EPERM,
            RusixErrno::ENOENT,
            RusixErrno::ESRCH,
            RusixErrno::EINTR,
            RusixErrno::EIO,
            RusixErrno::ENXIO,
            RusixErrno::E2BIG,
            RusixErrno::ENOEXEC,
            RusixErrno::EBADF,
            RusixErrno::ECHILD,
            RusixErrno::EAGAIN,
            RusixErrno::ENOMEM,
            RusixErrno::EACCES,
            RusixErrno::EFAULT,
            RusixErrno::ENOTBLK,
            RusixErrno::EBUSY,
            RusixErrno::EEXIST,
            RusixErrno::EXDEV,
            RusixErrno::ENODEV,
            RusixErrno::ENOTDIR,
            RusixErrno::EISDIR,
            RusixErrno::EINVAL,
            RusixErrno::ENFILE,
            RusixErrno::EMFILE,
            RusixErrno::ENOTTY,
            RusixErrno::ETXTBSY,
            RusixErrno::EFBIG,
            RusixErrno::ENOSPC,
            RusixErrno::ESPIPE,
            RusixErrno::EROFS,
            RusixErrno::EMLINK,
            RusixErrno::EPIPE,
            RusixErrno::EDOM,
            RusixErrno::ERANGE,
            RusixErrno::EDEADLK,
            RusixErrno::ENAMETOOLONG,
            RusixErrno::ENOLCK,
            RusixErrno::ENOSYS,
            RusixErrno::ENOTEMPTY,
            RusixErrno::ELOOP,
            RusixErrno::ENOMSG,
            RusixErrno::EIDRM,
            RusixErrno::ECHRNG,
            RusixErrno::EL2NSYNC,
            RusixErrno::EL3HLT,
            RusixErrno::EL3RST,
            RusixErrno::ELNRNG,
            RusixErrno::EUNATCH,
            RusixErrno::ENOCSI,
            RusixErrno::EL2HLT,
            RusixErrno::EBADE,
            RusixErrno::EBADR,
            RusixErrno::EXFULL,
            RusixErrno::ENOANO,
            RusixErrno::EBADRQC,
            RusixErrno::EBADSLT,
            RusixErrno::EBFONT,
            RusixErrno::ENOSTR,
            RusixErrno::ENODATA,
            RusixErrno::ETIME,
            RusixErrno::ENOSR,
            RusixErrno::ENONET,
            RusixErrno::ENOPKG,
            RusixErrno::EREMOTE,
            RusixErrno::ENOLINK,
            RusixErrno::EADV,
            RusixErrno::ESRMNT,
            RusixErrno::ECOMM,
            RusixErrno::EPROTO,
            RusixErrno::EMULTIHOP,
            RusixErrno::EDOTDOT,
            RusixErrno::EBADMSG,
            RusixErrno::EOVERFLOW,
            RusixErrno::ENOTUNIQ,
            RusixErrno::EBADFD,
            RusixErrno::EREMCHG,
            RusixErrno::ELIBACC,
            RusixErrno::ELIBBAD,
            RusixErrno::ELIBSCN,
            RusixErrno::ELIBMAX,
            RusixErrno::ELIBEXEC,
            RusixErrno::EILSEQ,
            RusixErrno::ERESTART,
            RusixErrno::ESTRPIPE,
            RusixErrno::EUSERS,
            RusixErrno::ENOTSOCK,
            RusixErrno::EDESTADDRREQ,
            RusixErrno::EMSGSIZE,
            RusixErrno::EPROTOTYPE,
            RusixErrno::ENOPROTOOPT,
            RusixErrno::EPROTONOSUPPORT,
            RusixErrno::ESOCKTNOSUPPORT,
            RusixErrno::EOPNOTSUPP,
            RusixErrno::EPFNOSUPPORT,
            RusixErrno::EAFNOSUPPORT,
            RusixErrno::EADDRINUSE,
            RusixErrno::EADDRNOTAVAIL,
            RusixErrno::ENETDOWN,
            RusixErrno::ENETUNREACH,
            RusixErrno::ENETRESET,
            RusixErrno::ECONNABORTED,
            RusixErrno::ECONNRESET,
            RusixErrno::ENOBUFS,
            RusixErrno::EISCONN,
            RusixErrno::ENOTCONN,
            RusixErrno::ESHUTDOWN,
            RusixErrno::ETOOMANYREFS,
            RusixErrno::ETIMEDOUT,
            RusixErrno::ECONNREFUSED,
            RusixErrno::EHOSTDOWN,
            RusixErrno::EHOSTUNREACH,
            RusixErrno::EALREADY,
            RusixErrno::EINPROGRESS,
            RusixErrno::ESTALE,
            RusixErrno::EUCLEAN,
            RusixErrno::ENOTNAM,
            RusixErrno::ENAVAIL,
            RusixErrno::EISNAM,
            RusixErrno::EREMOTEIO,
            RusixErrno::EDQUOT,
            RusixErrno::ENOMEDIUM,
            RusixErrno::EMEDIUMTYPE,
            RusixErrno::ECANCELED,
            RusixErrno::ENOKEY,
            RusixErrno::EKEYEXPIRED,
            RusixErrno::EKEYREVOKED,
            RusixErrno::EKEYREJECTED,
            RusixErrno::EOWNERDEAD,
            RusixErrno::ENOTRECOVERABLE,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<RusixErrno>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("RusixErrno", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for RusixErrno {
}

impl ::protobuf::reflect::ProtobufValue for RusixErrno {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x10protos/err.proto*\xd6\r\n\nRusixErrno\x12\t\n\x05EPERM\x10\x01\x12\
    \n\n\x06ENOENT\x10\x02\x12\t\n\x05ESRCH\x10\x03\x12\t\n\x05EINTR\x10\x04\
    \x12\x07\n\x03EIO\x10\x05\x12\t\n\x05ENXIO\x10\x06\x12\t\n\x05E2BIG\x10\
    \x07\x12\x0b\n\x07ENOEXEC\x10\x08\x12\t\n\x05EBADF\x10\t\x12\n\n\x06ECHI\
    LD\x10\n\x12\n\n\x06EAGAIN\x10\x0b\x12\n\n\x06ENOMEM\x10\x0c\x12\n\n\x06\
    EACCES\x10\r\x12\n\n\x06EFAULT\x10\x0e\x12\x0b\n\x07ENOTBLK\x10\x0f\x12\
    \t\n\x05EBUSY\x10\x10\x12\n\n\x06EEXIST\x10\x11\x12\t\n\x05EXDEV\x10\x12\
    \x12\n\n\x06ENODEV\x10\x13\x12\x0b\n\x07ENOTDIR\x10\x14\x12\n\n\x06EISDI\
    R\x10\x15\x12\n\n\x06EINVAL\x10\x16\x12\n\n\x06ENFILE\x10\x17\x12\n\n\
    \x06EMFILE\x10\x18\x12\n\n\x06ENOTTY\x10\x19\x12\x0b\n\x07ETXTBSY\x10\
    \x1a\x12\t\n\x05EFBIG\x10\x1b\x12\n\n\x06ENOSPC\x10\x1c\x12\n\n\x06ESPIP\
    E\x10\x1d\x12\t\n\x05EROFS\x10\x1e\x12\n\n\x06EMLINK\x10\x1f\x12\t\n\x05\
    EPIPE\x10\x20\x12\x08\n\x04EDOM\x10!\x12\n\n\x06ERANGE\x10\"\x12\x0b\n\
    \x07EDEADLK\x10#\x12\x10\n\x0cENAMETOOLONG\x10$\x12\n\n\x06ENOLCK\x10%\
    \x12\n\n\x06ENOSYS\x10&\x12\r\n\tENOTEMPTY\x10'\x12\t\n\x05ELOOP\x10(\
    \x12\n\n\x06ENOMSG\x10*\x12\t\n\x05EIDRM\x10+\x12\n\n\x06ECHRNG\x10,\x12\
    \x0c\n\x08EL2NSYNC\x10-\x12\n\n\x06EL3HLT\x10.\x12\n\n\x06EL3RST\x10/\
    \x12\n\n\x06ELNRNG\x100\x12\x0b\n\x07EUNATCH\x101\x12\n\n\x06ENOCSI\x102\
    \x12\n\n\x06EL2HLT\x103\x12\t\n\x05EBADE\x104\x12\t\n\x05EBADR\x105\x12\
    \n\n\x06EXFULL\x106\x12\n\n\x06ENOANO\x107\x12\x0b\n\x07EBADRQC\x108\x12\
    \x0b\n\x07EBADSLT\x109\x12\n\n\x06EBFONT\x10;\x12\n\n\x06ENOSTR\x10<\x12\
    \x0b\n\x07ENODATA\x10=\x12\t\n\x05ETIME\x10>\x12\t\n\x05ENOSR\x10?\x12\n\
    \n\x06ENONET\x10@\x12\n\n\x06ENOPKG\x10A\x12\x0b\n\x07EREMOTE\x10B\x12\
    \x0b\n\x07ENOLINK\x10C\x12\x08\n\x04EADV\x10D\x12\n\n\x06ESRMNT\x10E\x12\
    \t\n\x05ECOMM\x10F\x12\n\n\x06EPROTO\x10G\x12\r\n\tEMULTIHOP\x10H\x12\
    \x0b\n\x07EDOTDOT\x10I\x12\x0b\n\x07EBADMSG\x10J\x12\r\n\tEOVERFLOW\x10K\
    \x12\x0c\n\x08ENOTUNIQ\x10L\x12\n\n\x06EBADFD\x10M\x12\x0b\n\x07EREMCHG\
    \x10N\x12\x0b\n\x07ELIBACC\x10O\x12\x0b\n\x07ELIBBAD\x10P\x12\x0b\n\x07E\
    LIBSCN\x10Q\x12\x0b\n\x07ELIBMAX\x10R\x12\x0c\n\x08ELIBEXEC\x10S\x12\n\n\
    \x06EILSEQ\x10T\x12\x0c\n\x08ERESTART\x10U\x12\x0c\n\x08ESTRPIPE\x10V\
    \x12\n\n\x06EUSERS\x10W\x12\x0c\n\x08ENOTSOCK\x10X\x12\x10\n\x0cEDESTADD\
    RREQ\x10Y\x12\x0c\n\x08EMSGSIZE\x10Z\x12\x0e\n\nEPROTOTYPE\x10[\x12\x0f\
    \n\x0bENOPROTOOPT\x10\\\x12\x13\n\x0fEPROTONOSUPPORT\x10]\x12\x13\n\x0fE\
    SOCKTNOSUPPORT\x10^\x12\x0e\n\nEOPNOTSUPP\x10_\x12\x10\n\x0cEPFNOSUPPORT\
    \x10`\x12\x10\n\x0cEAFNOSUPPORT\x10a\x12\x0e\n\nEADDRINUSE\x10b\x12\x11\
    \n\rEADDRNOTAVAIL\x10c\x12\x0c\n\x08ENETDOWN\x10d\x12\x0f\n\x0bENETUNREA\
    CH\x10e\x12\r\n\tENETRESET\x10f\x12\x10\n\x0cECONNABORTED\x10g\x12\x0e\n\
    \nECONNRESET\x10h\x12\x0b\n\x07ENOBUFS\x10i\x12\x0b\n\x07EISCONN\x10j\
    \x12\x0c\n\x08ENOTCONN\x10k\x12\r\n\tESHUTDOWN\x10l\x12\x10\n\x0cETOOMAN\
    YREFS\x10m\x12\r\n\tETIMEDOUT\x10n\x12\x10\n\x0cECONNREFUSED\x10o\x12\r\
    \n\tEHOSTDOWN\x10p\x12\x10\n\x0cEHOSTUNREACH\x10q\x12\x0c\n\x08EALREADY\
    \x10r\x12\x0f\n\x0bEINPROGRESS\x10s\x12\n\n\x06ESTALE\x10t\x12\x0b\n\x07\
    EUCLEAN\x10u\x12\x0b\n\x07ENOTNAM\x10v\x12\x0b\n\x07ENAVAIL\x10w\x12\n\n\
    \x06EISNAM\x10x\x12\r\n\tEREMOTEIO\x10y\x12\n\n\x06EDQUOT\x10z\x12\r\n\t\
    ENOMEDIUM\x10{\x12\x0f\n\x0bEMEDIUMTYPE\x10|\x12\r\n\tECANCELED\x10}\x12\
    \n\n\x06ENOKEY\x10~\x12\x0f\n\x0bEKEYEXPIRED\x10\x7f\x12\x10\n\x0bEKEYRE\
    VOKED\x10\x80\x01\x12\x11\n\x0cEKEYREJECTED\x10\x81\x01\x12\x0f\n\nEOWNE\
    RDEAD\x10\x82\x01\x12\x14\n\x0fENOTRECOVERABLE\x10\x83\x01B\x02H\x01\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
