// Mr. Pavlov appears to be emulating interfaces by using structs
// containing function pointers according to his comment on the
// relevant section in 7zTypes.h:
//     The following interfaces use first parameter as pointer to structure

/// 7zTypes.h: collection of symbols SZ_ERROR_*
#[repr(i32)]
enum SevenZipErrorKind {
    /// 7zTypes.h: symbol SZ_ERROR_DATA
    Data = 1,

    /// 7zTypes.h: symbol SZ_ERROR_MEM
    Memory = 2,

    /// 7zTypes.h: symbol SZ_ERROR_CRC
    CRC = 3,

    /// 7zTypes.h: symbol SZ_ERROR_UNSUPPORTED
    Unsupported = 4,

    /// 7zTypes.h: symbol SZ_ERROR_PARAM
    Param = 5,

    /// 7zTypes.h: symbol SZ_ERROR_INPUT_EOF
    InputEOF = 6,

    /// 7zTypes.h: symbol SZ_ERROR_OUTPUT_EOF
    OutputEOF = 7,

    /// 7zTypes.h: symbol SZ_ERROR_READ
    Read = 8,

    /// 7zTypes.h: symbol SZ_ERROR_WRITE
    Write = 9,

    /// 7zTypes.h: symbol SZ_ERROR_PROGRESS
    Progress = 10,

    /// 7zTypes.h: symbol SZ_ERROR_FAIL
    Fail = 11,

    /// 7zTypes.h: symbol SZ_ERROR_THREAD
    Thread = 12,

    /// 7zTypes.h: symbol SZ_ERROR_ARCHIVE
    Archive = 16,

    /// 7zTypes.h: symbol SZ_ERROR_NO_ARCHIVE
    NoArchive = 17
}

/// 7zTypes.h: typedef SRes
type SRes = Result<(), SevenZipErrorKind>;

/// 7zTypes.h: struct IByteIn
trait IByteIn {
    /// Byte (*Read)(const IByteIn *p)
    fn read(&mut self) -> u8;
}

/// 7zTypes.h: struct IByteOut
trait IByteOut {
    /// void (*Write)(const IByteOut *p, Byte b)
    fn write(&mut self, b: u8);
}

/// 7zTypes.h: struct ISeqInStream
trait ISeqInStream {
    /// SRes (*Read)(const ISeqInStream *p, void *buf, size_t *size);
    fn read<T>(&mut self, buf: &Vec<T>) -> SRes;
}

/// 7zTypes.h: struct ISeqInStream
trait ISeqOutStream {
    /// size_t (*Write)(const ISeqOutStream *p, const void *buf, size_t size);
    fn write<T>(&mut self, buf: &Vec<T>) -> usize;
}

/// 7zTypes.h: typedef enum { ... } ESzSeek
#[repr(i32)]
enum SeekKind {
    /// SZ_SEEK_SET
    Set = 0,

    /// SZ_SEEK_CUR
    Current = 1,

    /// SZ_SEEK_END
    End = 2
}

/// 7zTypes.h: struct ISeekInStream
trait ISeekInStream {
    /// SRes (*Read)(const ISeekInStream *p, void *buf, size_t *size)
    fn read<T>(&mut self, buf: &Vec<T>) -> SRes;

    /// SRes (*Seek)(const ISeekInStream *p, Int64 *pos, ESzSeek origin)
    fn seek(&mut self, position: usize, origin: SeekKind);
}

/// 7zTypes.h: struct ILookInStream
trait ILookInStream {
    /// SRes (*Look)(const ILookInStream *p, const void **buf, size_t *size);
    fn look<T>(&mut self, buf: &Vec<T>) -> SRes; // could be &mut Vec<T>

    /// SRes (*Skip)(const ILookInStream *p, size_t offset);
    fn skip(&mut self, offset: usize) -> SRes;

    /// SRes (*Read)(const ILookInStream *p, void *buf, size_t *size);
    fn read<T>(&mut self, buf: &Vec<T>) -> SRes;

    /// SRes (*Seek)(const ILookInStream *p, Int64 *pos, ESzSeek origin);
    fn seek(&mut self, pos: i64, origin: SeekKind) -> SRes;
}

/// 7zTypes.h: struct ICompressProgress
trait ICompressProgress {
    /// SRes (* Progress)(const ICompressProgress* p, UInt64 inSize, UInt64 outSize)
    fn progress(in_size: u64, out_size: u64) -> SRes;
}

/// 7zTypes.h: struct CLookToRead2
struct CLookToRead2 {
    pos: usize,
    size: usize,
    buffer: Vec<u8>,
    lookahead: bool
}

impl CLookToRead2 {
    pub fn new(lookahead: bool) -> Self {
        Self {
            pos: 0,
            size: 0,
            buffer: vec![],
            lookahead
        }
    }
}

// TODO: Let's just leave this as-is for now and come back later since there are more ridiculous macros here...
impl ILookInStream for CLookToRead2 {
    fn look<T>(&mut self, buf: &Vec<T>) -> SRes {
        todo!()
    }

    fn skip(&mut self, offset: usize) -> SRes {
        todo!()
    }

    fn read<T>(&mut self, buf: &Vec<T>) -> SRes {
        todo!()
    }

    fn seek(&mut self, pos: i64, origin: SeekKind) -> SRes {
        todo!()
    }
}


