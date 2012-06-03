// Kevin Cantu
// hash function testing

// note, when either of the hashes is used with inlining, and
// this file is optimized, this comparison becomes junk

use std;
use murmur;
use djb;

fn word_of_god () -> [str]
{
   io::println ("Loading the lolcat bible...");
   let path = "./lolcat/LOLCatBible_2012-01-04.xml";
   let bible = io::read_whole_file (path);
   let bible_ = str::from_bytes (result::get (bible));

   ret str::words(bible_) +
       vec::map(str::lines(bible_), {|ln| ln+ln+ln+ln});
}

// given a string hash function, test its behavior
fn hash_bench <TT> ( label: str, hashfn: native fn(&&str)->TT, data: [str] )
{
   let t0 = std::time::precise_time_s();
   let _v = vec::map(data, hashfn);
   let t1 = std::time::precise_time_s();

   io::println(label + #fmt("%06.3f sec", t1 - t0));
}


// main
fn main () {
   let meow = word_of_god();

   hash_bench ("Benching djb...     ", djb::djb,          meow);
   hash_bench ("Benching murmur3... ", murmur::murmur,       meow);
}


