use std::path::PathBuf;

#[derive(Debug)]
pub enum Error {
    UnsupportedMajorVersion,
    UnsupportedMinorVersion,
    InvalidMagicNumber,
    FailedToOpen(std::io::Error),
    FailedToReadMetaData(std::io::Error),
    FailedToMapToMemory(nix::errno::Errno),
    ConversionError(Vec<u8>),
    StringConversionError(std::string::FromUtf8Error),
    FailedToSync(nix::errno::Errno),
    InvalidPath(PathBuf),
    FailedToParsePid(std::num::ParseIntError),
    WontBeAbleToRead,
    OffsetOutOfBounds,
    NotAlignedForCOnversion,
}
