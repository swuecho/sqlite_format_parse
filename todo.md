# why assert_eq!(len, Ok(100)) is not right?

# FAQ: how to print a byte as char
    
   let file_header = [83,84]
   for byte in  &file_header[0..2] {
       println!("{}", *byte as char);
   }

# array slice

 let array = [1,2,3]

slice all elements, &array[..]
slice serveral, &array[start..end], end is not included
 
