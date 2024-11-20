use std::net::TcpStream;
use std::collections::HashMap;
use std::io::Write;

#[derive(Debug, PartialEq, Eq)]
pub enum HttpTypes {
   Post,
   Get
}
#[derive(Debug)]
pub struct RequestType {
   http_type: HttpTypes,
   request: String,
}


pub fn handle_connection(mut stream: TcpStream) {
     let possible_connection: Option<(RequestType,String, HashMap<String, String>)> =   get_body_and_headers(&mut stream);
     if let None = possible_connection {
        return ();
     } else{
        let (request,_,_) = possible_connection.unwrap();
    
        if request.http_type == HttpTypes::Get {

           let status_line = "HTTP/1.1 200 OK";
           let contents = std::fs::read_to_string("websites/Main.html").unwrap();
           let length = contents.len();
         
           let response =
           format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
           stream.write_all(response.as_bytes()).unwrap();
        }
        else {
           let status_line = "HTTP/1.1 200 OK";
           let response =format!("{status_line}\r\nContent-Length: 0\r\n\r\n").as_bytes().to_vec() ;
           _ = stream.write_all(&response);
     
        }
    }
}


pub fn get_body_and_headers(stream: &mut TcpStream) -> Option<(RequestType, String, HashMap<String, String>)> { 
    let mut buf = [0; 10000]; // TODO figure this shit.
    if let Ok(len) = stream.peek(&mut buf) {
      let mut buf = vec![0;len];
      let _ = stream.peek(&mut buf).unwrap();
   
      if let Ok(whole_request) = String::from_utf8(buf.to_vec()) {
        let mut header_str: String = String::new();
        let mut header = HashMap::new();
        let mut request: RequestType = RequestType{http_type : HttpTypes::Get, request: String::new()} ;
    
        let mut lines = whole_request.lines();
     
        loop {
           let line = lines.next();
           match line {
              Some(line) => {
              if line.len() < 3 {
                 break;
              }
           
              header_str.push_str( line);
              header_str.push('\n');
           },
              None => return None,
           }
        }
     
     
        for (i, line) in header_str.lines().into_iter() .enumerate() {
           let thing: Vec<&str> = line.split(" ").collect();
           if i == 0 {
              match thing.get(0) {
                 Some(t) =>  {
                       match t {
                          &"GET" => {request.http_type = HttpTypes::Get}
                          &"POST" => {request.http_type = HttpTypes::Post}
     
     
                          _ => {return None},
                       }
                 },
                 None => {return None}
              }
     
              match thing.get(1) {
                 Some(t) => {request.request = t.to_string() },
                 None => {return None},
              }
           }
           else {
              let mut x = thing.get(0).unwrap_or(&"").to_string();
              let y = thing.get(1).unwrap_or(&"").to_string();
     
              if !x.is_empty() && !y.is_empty() {
     
                 x.pop();
     
                 header.insert(x,y);
              }
           }
        }
     
     
        let mut body = String::new();
        for line in lines {
            body.push_str(line);
        }
       
        
        return Some( (request, body, header ) )
      };
    } 
    
    
    return None
}