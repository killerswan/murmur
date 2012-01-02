// Kevin Cantu
// replace djb with murmur
// https://github.com/graydon/rust/blob/master/src/libcore/str.rs at ll. 43-50

use std;
import std::io;

fn displayHash(label: str,
               hf: fn(&&str)->uint,
               ss: str
              ) {
   io::println (#fmt ("%s: \"%s\" -> %u", label, ss, hf(ss) ) );
}

fn djb(&&s: str) -> uint {
   let u: uint = 5381u;
   for c: u8 in s { u *= 33u; u += c as uint; }
   ret u;
}

fn murmur(&&s: str) -> uint {
   ret 1u;
}

fn murmur_str(&&s: str) -> str {
   ret "ABCD";
}

fn main () {
   let ss = "banana";

   displayHash("djb   ", djb, ss);
   displayHash("murmur", murmur, ss);
}


