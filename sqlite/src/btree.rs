

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
pub struct FileHeader<'a> {
 pub   header_string: &'a str,
 pub   page_size: u16,
 pub   file_format_write_version: u8,
 pub   file_format_read_version: u8,
 pub   bytes_unused_per_page: u8, 
 pub   max_embedded_payload_fraction: u8,
 pub   min_embedded_payload_fraction: u8,
 pub   min_leaf_palyload_fraction:u8,
 pub   file_change_counter: u32,
 pub   first_freelist_page: u32,
 pub    number_freelist_page: u32,
}

