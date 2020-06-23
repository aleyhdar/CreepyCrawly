extern crate reqwest;
fn main() {
   //Thanks dcode
   let response_text = reqwest::get("https://en.wikipedia.org/wiki/Thread_safety")
        .expect("Could not resp text!")
        .text().expect("Could not the reach site");
   /*
   match reqwest::get("https://en.wikipedia.org/wiki/Thread_safety"){
        Ok(mut response) => {
            if response.status() == reqwest::StatusCode::Ok {
                match response.text() {
                    Ok(text) => println!("Resp text: {}", text),
                    Err(_) => println!("Could not read resp text!")
                }
            } else {
                println!("Unable to connect!");
            }
       }
       Err(_) => println!("Could not find a site!")
   }*/
}
