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
   let t0 = std::time::get_time();
   let _v = vec::map(data, hashfn);
   let t1 = std::time::get_time();

   let (ds,du) = (t1.sec-t0.sec, t1.usec-t0.usec);

   std::io::println(label + #fmt("%u sec %u usec", ds as uint, du as uint));
}


// main
fn main () {
   std::io::println ("");

   let meow = word_of_god();

   hash_bench ("Benching dummy_hash...  ", {|_x| "ABCD"},      meow);
   hash_bench ("Benching djb...         ", murmur::djb,        meow);
   hash_bench ("Benching murmur3...     ", murmur::murmur, meow);
}


