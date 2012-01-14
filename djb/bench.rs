// Kevin Cantu
// hash function testing

#[link(name = "bench", author = "kcantu", vers = "0.0")];

use std;
use str2;

fn word_of_god () -> [str]
{
   std::io::println ("Loading the lolcat bible...");
   let path = "../lolcat/LOLCatBible_2012-01-04.xml";
   let bible = std::io::read_whole_file (path);
   let bible_ = str::unsafe_from_bytes (result::get (bible));

   ret str2::words(bible_) +
       vec::map(str2::lines(bible_), {|ln| ln+ln+ln+ln});
}

fn hash_bench <TT> ( label: str, hashfn: fn(&&str)->TT, data: [str] )
{
   let t0 = std::time::precise_time_s();
   let _v = vec::map(data, hashfn);
   let t1 = std::time::precise_time_s();

   std::io::println(label + #fmt("%06.3f sec", t1 - t0));
}


