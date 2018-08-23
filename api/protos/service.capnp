@0x882a88c23601ee75;

enum ResultType {
	ok  @0;
	err  @1;
}

struct OpResult {
	result  @0 :ResultType;
	# error_msg is set if ERR
	errno  @1 :Int32;
	errorMsg  @2 :Text;
}

struct Iatt {
	iaGfid  @0 :Text;
	iaIno  @1 :UInt64;
	iaDev  @2 :UInt64;
	mode  @3 :UInt32;
	iaNlink  @4 :UInt32;
	iaUid  @5 :UInt32;
	iaGid  @6 :UInt32;
	iaRdev  @7 :UInt64;
	iaSize  @8 :UInt64;
	iaBlksize  @9 :UInt32;
	iaBlocks  @10 :UInt64;
	iaAtime  @11 :UInt32;
	iaAtimeNsec  @12 :UInt32;
	iaMtime  @13 :UInt32;
	iaMtimeNsec  @14 :UInt32;
	iaCtime  @15 :UInt32;
	iaCtimeNsec  @16 :UInt32;
}

struct StatRequest {
	gfid  @0 :Text;
	extraData  @1 :Data;
}

struct StatResponse {
	result  @0 :OpResult;
	stat  @1 :Iatt;
	extraData  @2 :Data;
}
