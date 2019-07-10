#![allow(unused)]

// Describes a file within a zip file.
pub struct FileHeader {
    // Name of the file
    // Must be a relative path, not start with a drive letter (e.g. C:),
    // and must use forward slashes instead of back slashes
    pub name: String,

    // Comment is any arbitrary user-defined string shorter than 64KiB
    pub comment: Option<String>,

    pub non_utf8: bool,

    pub creator_version: u16,
    pub reader_version: u16,
    pub flags: u16,

    pub modified: chrono::DateTime<chrono::offset::Utc>,

    pub crc32: u32,
    pub compressed_size: u64,
    pub uncompressed_size: u64,

    pub extra: Option<Vec<u8>>,
    pub external_attrs: u32,
}

// Compression method
#[repr(u16)]
#[derive(Debug)]
pub enum Method {
    Store = 0,
    Deflate = 8,
    BZIP2 = 12,
    LZMA = 14,
}

impl FileHeader {
    pub fn new<S>(name: S, uncompressed_size: u64, method: Method) -> Self
    where
        S: Into<String>,
    {
        Self {
            name: name.into(),
            comment: None,
            non_utf8: false,

            creator_version: ZipVersion::Version45 as u16,
            reader_version: ZipVersion::Version45 as u16,
            flags: 0,

            modified: chrono::DateTime::from_utc(
                chrono::naive::NaiveDateTime::from_timestamp(0, 0),
                chrono::offset::Utc,
            ),

            crc32: 0,
            compressed_size: 0,
            uncompressed_size: 0,

            extra: None,
            external_attrs: 0,
        }
    }
}

/// Constants for the first byte in creator_version
#[repr(u8)]
enum CreatorVersion {
    FAT = 0,
    Unix = 3,
    NTFS = 11,
    VFAT = 14,
    MacOSX = 19,
}

/// Version numbers
#[repr(u8)]
enum ZipVersion {
    /// 2.0
    Version20 = 20,
    /// 4.5 (reads and writes zip64 archives)
    Version45 = 45,
}