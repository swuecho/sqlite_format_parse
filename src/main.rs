extern crate sqlite;

use std::fs::File;
use std::io::Read;
use std::str;
use sqlite::util::*;
use sqlite::btree::*;


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
