

fn main() {
   // println!("Hello, world!")
   if let Err(e) = catr::get_user_args().and_then(catr::run) {
    eprintln!("{}", e);

    std::process::exit(1);
   }
}



