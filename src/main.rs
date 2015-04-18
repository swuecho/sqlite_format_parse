use std::fs::File;
use std::io::Read;
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
/*  TODO
**
**     40       4     Schema cookie
**     44       4     File format of schema layer
**     48       4     Size of page cache
**     52       4     Largest root-page (auto/incr_vacuum)
**     56       4     1=UTF-8 2=UTF16le 3=UTF16be
**     60       4     User version
**     64       4     Incremental vacuum mode
**     68       4     Application-ID
**     72      20     unused
**     92       4     The version-valid-for number
**     96       4     SQLITE_VERSION_NUMBER
**
*/
fn main() {
   let file_name = "./data/words.db"; 
   let mut sqlite_file = File::open(file_name).unwrap();

   let sqlite_file_size = sqlite_file.metadata().unwrap().len();

   println!("{:?}", sqlite_file_size);

   let mut file_header_arr = [0u8; 100]; 
   let len = sqlite_file.read(&mut file_header_arr); 
   let file_header = parse_file_header(&file_header_arr);
   println!("{:?}", len);

   println!("{:?}", file_header);
}

// convert 2 byte u8 to a u16
fn get2byte(p: &[u8]) -> u16 {
  assert_eq!(p.len(), 2);
  (p[0] as u16) << 8u8  | p[1] as u16
}

// convert 4 byte u8 to a u32
fn get4byte(p: &[u8]) -> u32 {
    // how to case each u8 in array?
    assert_eq!(p.len(), 4);
    (p[0] as u32) <<24 | (p[1] as u32) <<16 | (p[2] as u32) << 8 | p[3] as u32
}


fn parse_file_header(file_header_arr: &[u8;100]) -> FileHeader {

   let header_string_sqlite = str::from_utf8(&file_header_arr[0..15]).unwrap();

   FileHeader {
        header_string: header_string_sqlite,
        page_size:  get2byte(&file_header_arr[16..18]),
        file_format_write_version: file_header_arr[18],
        file_format_read_version: file_header_arr[19],
        bytes_unused_per_page: file_header_arr[20], 
        max_embedded_payload_fraction: file_header_arr[21],
        min_embedded_payload_fraction: file_header_arr[22],
        min_leaf_palyload_fraction: file_header_arr[23],
        file_change_counter:  get4byte(&file_header_arr[24..28]),
        first_freelist_page:  get4byte(&file_header_arr[32..36]),
        number_freelist_page:  get4byte(&file_header_arr[36..40]),
    }

}

/*
fun parse_page_hader {
  let mut page_header = [0u8; 20]; 
  let page_header_len = sqlite_file.read(&mut page_header); 
  let leaves = page_heaer[0..8];
  let nodes = page_header[8..20];
  for byte in  &page_header[..] {
       println!("{}", byte);
   }
}
*/
