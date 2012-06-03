// a copy of the core library's str::hash()

#[link(name = "djb", vers = "0")];

fn djb(&&s: str) -> uint {
   let mut u: uint = 5381u;
   str::bytes_iter(s, { |c| u *= 33u; u += c as uint; });
   ret u;
}


