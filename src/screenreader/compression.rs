pub fn compress_frame(data :Vec<u8>) -> Vec<u8> {
    let mut compressed_data: Vec<u8> = Vec::new();
 
    compressed_data.push(data[0]);
    compressed_data.push(data[1]);
    compressed_data.push(data[2]);
    compressed_data.push(data[3]);
    let mut i: usize = 4;
    while i < data.len() {
        let  byte = data[i];
 
        i += 1;
        let mut repeat = 0;
        for _ in 0..255 {
            let furtherbyteo = data.get(i);
            match furtherbyteo {
                Some(furtherbyte) => {
                    if furtherbyte == &byte {
                        repeat += 1;
                        i+=1;
       
                    } else {
                        break;
                    }
                }
                None => {break}
            }
        }
 
 
 
       // if repeat > 1 {
          compressed_data.push(byte + 1);
          compressed_data.push(repeat);
       // } else {
 
 
        //  compressed_data.push(byte)
       // }
       
    }
 
 
 
 
    return compressed_data
 }
 