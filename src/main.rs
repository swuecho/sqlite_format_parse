use std::fs::File;
use std::io::Read;
use std::str;
//use std::ascii::AsciiExt;
//use std::collections::HashMap;
//use std::borrow::ToOwned;

/*
** The first page is always a btree page.  The first 100 bytes of the first
** page contain a special header (the "file header") that describes the file.
** The format of the file header is as follows:
**
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
   let mut file_header = [0u8; 100]; 
   let sqlite_file_size = sqlite_file.metadata().unwrap().len();
   let len = sqlite_file.read(&mut file_header); 
   // check the len is Ok(100);
   println!("{:?}", len);
   println!("{:?}", sqlite_file_size);
  // why assert_eq!(len, Ok(100)) is not right?

   /* FAQ: how to print a byte as char
   for byte in  &file_header[0..15] {
       println!("{}", *byte as char);
   }
   */

   let header_string = str::from_utf8(&file_header[0..15]);

   println!("{:?}", header_string);
   let page_size = file_header[17] as u16
       		    + (file_header[16] as u16) << 8u8 ;
   println!("the page size is {}", page_size);
   println!("{}", (sqlite_file_size)  / page_size as u64);
   /*
   for byte in  &file_header[16..18] {
       println!("{}", byte);
   }
   */

//   let mut page_header = [0u8; 20]; 
 //  let page_header_len = sqlite_file.read(&mut page_header); 
  //   let leaves = page_heaer[0..8];
  //  let nodes = page_header[8..20];
 /*  for byte in  &page_header[..] {
       println!("{}", byte);
   }
   */

}
