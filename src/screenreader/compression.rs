pub fn compress_frame(data :Vec<u8>,previous_data: &Vec<u8>,ignore_previous_data: bool) -> Vec<u8> {
    let mut compressed_data: Vec<u8> = Vec::new();
 
    compressed_data.push(data[0]);
    compressed_data.push(data[1]);
    compressed_data.push(data[2]);
    compressed_data.push(data[3]);

    let mut i: usize = 4;
    while i < data.len() {
        let  byte1 = data[i];
        let  byte2 = data[i + 1];
        if !ignore_previous_data {
            let previous1 = previous_data[i]; 
            let previous2 = previous_data[i + 1];
            i += 2;
            if byte1 == previous1 && byte2 == previous2 {
                let mut repeat = 0;
                for _ in 0..127 {
                    let furtherbyteo1 = data.get(i);
                    let furtherbyteo2 = data.get(i + 1);
                    match furtherbyteo1 {
                        Some(furtherbyte1) => {
                            match furtherbyteo2 { 
                                Some(furtherbyte2 ) => {
                                    if *furtherbyte1 == previous_data[i] && *furtherbyte2 == previous_data[i + 1]{
                                        
                                        repeat += 1;
                                        i+=2;
                    
                                    } else {
                                        break;
                                    }
                                }
                                None => {break}
                            }
                        }
                        None => {break}
                    }
                }
    
                compressed_data.push(repeat * 2);
                continue    
            } 
        } else {
            i += 2;
        }
      

 
    

        let mut repeat = 0;
        
        for _ in 0..255 {
            let furtherbyteo1 = data.get(i);
            let furtherbyteo2 = data.get(i + 1);
            match furtherbyteo1 {
                Some(furtherbyte1) => {
                    match furtherbyteo2 { 
                        Some(furtherbyte2 ) => {
                            if *furtherbyte1 == byte1 && *furtherbyte2 == byte2{
                                
                                repeat += 1;
                                i+=2;
               
                            } else {
                                break;
                            }
                        }
                        None => {break}
                    }
                }
                None => {break}
            }
        }
 
        if repeat > 0 {
          compressed_data.push(byte1 + 2);
          compressed_data.push(byte2);
          compressed_data.push(repeat);
        } else {
          compressed_data.push(byte1);
          compressed_data.push(byte2);
        }
       
    }
 
 
    return compressed_data
 }
 