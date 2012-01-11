// Kevin Cantu
// hash function testing

use std;
use murmur;
use str2;

fn word_of_god () -> [str]
{
   let path = "./lolcat/LOLCatBible_2012-01-04.xml";
   let bible = std::io::read_whole_file (path);
   let bible_ = str::unsafe_from_bytes (result::get (bible));

   ret str2::words(bible_) + str2::lines(bible_);
}

// given a string hash function
// (modified to provide a string return value)
// test its behavior
fn hash_bench <TT> ( label: str, hashfn: fn(&&str)->TT, data: [str] )
{
   let t0 = std::time::precise_time_s();
   let _v = vec::map(data, hashfn);
   let t1 = std::time::precise_time_s();

   std::io::println(label + #fmt("%06.3f sec", t1 - t0));
}


// main
fn main () {
   let meow = word_of_god();

   hash_bench ("Benching djb...     ", murmur::djb,          meow);
   hash_bench ("Benching murmur3... ", murmur::murmur,       meow);
}

