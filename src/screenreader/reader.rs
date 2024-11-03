use captrs::Capturer;

pub fn makebase() -> Vec<u8> {
   let (xsize, ysize) = crate::settings::SCREEN_SIZE;
   let mut bytes: Vec<u8> = Vec::new();

   bytes.push((xsize / 256) as u8);
   bytes.push((xsize % 256) as u8);

   bytes.push((ysize / 256) as u8);
   bytes.push((ysize % 256) as u8);

   for _ in 0..xsize {
      for _ in 0..ysize {
         bytes.push(0);
         bytes.push(0);
      }
   }

   return bytes;
}

pub fn getmonitordata(capture:&mut Capturer) -> Option<Vec<u8>> {
    let (xsize, ysize) = crate::settings::SCREEN_SIZE;
   
   
    let result = capture.capture_store_frame();

    match result {
      Ok(_) => {}
      Err(_) => {return None}

    }

    let imageop = capture.get_stored_frame();
    let  image = imageop.unwrap();
  //  match imageop {
  //     Ok(t) => {image =  t;},
   //    Err(_) => {return None}
  //  }

    let (xscreen, yscreen) = capture.geometry();
 
    let mut screen_bytes: Vec<u8> = Vec::new();
    
    screen_bytes.push((xsize / 256) as u8);
    screen_bytes.push((xsize % 256) as u8);
 
    screen_bytes.push((ysize / 256) as u8);
    screen_bytes.push((ysize % 256) as u8);
    
 
    {
      let xscreen = xscreen as f32;
      let yscreen = yscreen as f32;
      let xsizef = xsize as f32;
      let ysizef = ysize as f32;
      for y in 0..ysize {
         let y = y as f32;
          for x in 0..xsize {
             let x = x as f32;
    
             let mut r: u16 = 0;
             let mut g: u16 = 0;
             let mut b: u16 = 0;
    
             for xc in -1..=1 {
                let rx: u32 =  ( (x + ((xc as f32) / 3f32))  / xsizef  * xscreen) as u32;
                for yc in -1..=1 {
                   let ry: u32 =  ( (y + ((yc as f32) / 3f32))  / ysizef  * yscreen) as u32;
                   unsafe {
                     let pixel =  image.get_unchecked((rx + ry * (xscreen as u32)) as usize);
                     
                     r += pixel.r as u16;
                     g += pixel.g as u16;
                     b += pixel.b as u16;
                   }            
                }
             }
    
             let r =  (r / 9).clamp(0, 255);
             let g =  (g / 9).clamp(0, 255);
             let b =  (b / 9).clamp(0, 255);
    
    
             //let darken = (r + g + b) / 3
   
             let r_reduced = ((r + 8) / 16).clamp(0,15); 
             let g_reduced = ((g + 8) / 16).clamp(0,15); 
             let b_reduced = ((b + 8) / 16).clamp(0,15); 
    
             // Binary: SSSS RRRR GGGG BBBB
   
             screen_bytes.push(((r_reduced * 16 + 1) ) as u8);
             screen_bytes.push((g_reduced + b_reduced * 16) as u8);
   
          }
       }
    }
   
    return Some( screen_bytes);
 }