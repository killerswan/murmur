// Kevin Cantu
// hash function testing

use std;
use bench;

fn splitfn(ss: str, sepf: block(cc: char)->bool) -> [str]
{
   let vv: [str] = [];
   let accum: str = "";
   let ends_with_sep: bool = false;

   str::iter_chars(ss, {|cc|
      if sepf(cc) {
         vv += [accum];
         accum = "";
         ends_with_sep = true;
      } else {
         str::push_char(accum, cc);
         ends_with_sep = false;
      }
   });

   if str::char_len(accum) != 0u || ends_with_sep {
      vv += [accum];
   }

   ret vv;
}

fn lines (ss: str) -> [str]
{
   ret splitfn(ss, {|cc| cc == '\n'});
}

fn words (ss: str) -> [str]
{
   ret vec::filter( splitfn(ss, {|cc| char::is_whitespace(cc)}), 
                    {|w| 0u < str::char_len(w)});
}

// hash fn being mapped
fn djb(&&s: str) -> uint
{
    let u: uint = 5381u;
    for c: u8 in s { u *= 33u; u += c as uint; }
    ret u;
}

fn main ()
{
   let meow = word_of_god();
   hash_bench ("calling hash_bench()...   ", djb, meow);
}


