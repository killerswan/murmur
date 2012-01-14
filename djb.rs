// a copy of the core library's str::hash()

#[link(name = "djb", vers = "0")];

fn djb(&&s: str) -> uint {
    let u: uint = 5381u;
    for c: u8 in s { u *= 33u; u += c as uint; }
    ret u;
}


