// Kevin Cantu
// hash function testing

use std;

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

////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////
// when BOTH of the following functions are compiled at opt level 1 instead of 0
// the program becomes SLOWER with optimization
fn word_of_god () -> [str]
{
   std::io::println ("Loading the lolcat bible...");
   let path = "../lolcat/LOLCatBible_2012-01-04.xml";
   let bible = std::io::read_whole_file (path);
   let bible_ = str::unsafe_from_bytes (result::get (bible));

   ret words(bible_) +
       vec::map(lines(bible_), {|ln| ln+ln+ln+ln});
}

fn hash_bench <TT> ( label: str, hashfn: native fn(&&str)->TT, data: [str] )
{
   let t0 = std::time::precise_time_s();
   let _v = vec::map(data, hashfn);
   let t1 = std::time::precise_time_s();

   std::io::println(label + #fmt("%06.3f sec", t1 - t0));
}
////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////


