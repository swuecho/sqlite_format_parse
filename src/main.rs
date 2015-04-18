use std::fs::File;
use std::io::Read;
use std::str;
use std::ascii::AsciiExt;
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
   let file_name = "/home/hwu/words.db"; 
   let mut sqlite_file = File::open(file_name).unwrap();
   let mut first_100_bytes = [0u8; 100]; 
   let len = sqlite_file.read(&mut first_100_bytes); 
       println!("{:?}", len);
  // why assert_eq!(len, Ok(100)) is not right?

   for byte in  &first_100_bytes[0..15] {
       println!("{}", *byte as char);
   }
    
   println!("{:?}",(str::from_utf8(&first_100_bytes[0..15])));

   for byte in  &first_100_bytes[16..18] {
       println!("{}", byte);
   }

}
