use captrs::Capturer;

pub fn getmonitordata(capture:&mut Capturer) -> Option<Vec<u8>> {
    let (xsize, ysize) = crate::settings::SCREEN_SIZE;

    let imageop = capture.capture_frame();
    let  image;
    match imageop {
       Ok(t) => {image =  t;},
       Err(_) => {return None}
    }

    let (xscreen, yscreen) = capture.geometry();
 
    let mut screen_bytes: Vec<u8> = Vec::new();
    
    screen_bytes.push((xsize / 256) as u8);
    screen_bytes.push((xsize % 256) as u8);
 
    screen_bytes.push((ysize / 256) as u8);
    screen_bytes.push((ysize % 256) as u8);
    
 
    for y in 0..ysize {
       for x in 0..xsize {
 
          let mut r: u16 = 0;
          let mut g: u16 = 0;
          let mut b: u16 = 0;
 
          for xc in -1..=1 {
             for yc in -1..=1 {
                let rx: u32 =  ( ((x as f32) + ((xc as f32) / 3f32))  / (xsize as f32) * (xscreen as f32)) as u32;
                let ry: u32 =  ( ((y as f32) + ((yc as f32) / 3f32))  / (ysize as f32) * (yscreen as f32)) as u32;            
                let pixel =  image.get((rx + ry * (xscreen as u32)) as usize).unwrap() ;
                r += pixel.r as u16;
                g += pixel.g as u16;
                b += pixel.b as u16;
             }
          }
 
          let r =  (r / 9).clamp(0, 255);
          let g =  (g / 9).clamp(0, 255);
          let b =  (b / 9).clamp(0, 255);
 
 
          //let darken = (r + g + b) / 3

          let r_reduced = ((r + 32) / 64).clamp(0,3); // max 3
          let g_reduced = ((g + 32) / 64).clamp(0,3); // max 3
          let b_reduced = ((b + 32) / 64).clamp(0,3); // max 3
 
          // Binary: HRRGGGBB


          // HLLLRRRS 
          screen_bytes.push((r_reduced * 4+ g_reduced * 16 + b_reduced * 64) as u8);
       }
    }
    return Some( screen_bytes);
 }