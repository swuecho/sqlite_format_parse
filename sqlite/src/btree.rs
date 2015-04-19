use std::str;
//use std::ascii::AsciiExt;
//use std::collections::HashMap;
//use std::borrow::ToOwned;

/*
**   OFFSET   SIZE    DESCRIPTION
**      0      16     Header string: "SQLite format 3\000"
**     16       2     Page size in bytes.  (1 means 65536)
**     18       1     File format write version
**     19       1     File format read version
**     20       1     Bytes of unused space at the end of each page
**     21       1     Max embedded payload fraction (must be 64)
**     22       1     Min embedded payload fraction (must be 32)
**     23       1     Min leaf payload fraction (must be 32)
**     24       4     File change counter
**     28       4     Reserved for future use
**     32       4     First freelist page
**     36       4     Number of freelist pages in the file
**     40      60     15 4-byte meta values passed to higher layers
*/


#[derive(Debug)]
struct FileHeader<'a> {
    header_string: &'a str,
    page_size: u16,
    file_format_write_version: u8,
    file_format_read_version: u8,
    bytes_unused_per_page: u8, 
    max_embedded_payload_fraction: u8,
    min_embedded_payload_fraction: u8,
    min_leaf_palyload_fraction:u8,
    file_change_counter: u32,
    first_freelist_page: u32,
    number_freelist_page: u32,
}

