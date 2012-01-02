// Kevin Cantu
// hash function testing
//
// given the appropriate files which contain key/value pairs...
// usage: rustx read-pairs.rs

use std;
use murmur;


// given a string hash function
// (modified to provide a string return value)
// test its behavior against a file of test keys and values
fn hash_test ( test_label: str,
                wrapped_hash: fn(&&str)->str,
                path: str
              ) {

   let res = std::io::read_whole_file (path);
   let data = str::unsafe_from_bytes (result::get (res));
   let lines = str::split_str (data, "\n");


   fn getKV (ss: str) -> [str] {
      let words = str::split_str (ss, ",");
      ret vec::map (words, {|s| str::trim(s)});
   }

   let keyVal = vec::map (
                   vec::filter (
                      vec::map (lines, {|s| getKV(s)}),
                      {|kk| vec::len(kk) == 2u}
                   ),
                   {|jj| (jj[0],jj[1])}
                );


   let pairsFailed = 0u;
   let pairsTotal = 0u;

   std::io::println (test_label);

   vec::map (keyVal, block (ab: (str,str)) -> () {

      let (key,val) = ab;
      let val_ = wrapped_hash(key);

      pairsTotal += 1u;

      if val != val_ {
         pairsFailed += 1u;

         std::io::println (#fmt("expected \"%s\"",  key ));
         std::io::println (#fmt("         -> 0x%s", val ));
         std::io::println (#fmt("        not 0x%s", val_));
      }
   });

   std::io::println ( #fmt(
      "%u/%u pairs failed\n", pairsFailed, pairsTotal));
}


// main
fn main () {
   std::io::println ("");

   hash_test ("Testing dummy_hash...",
              {|_x| "ABCD"}, "./reference/dummy.tests");

   hash_test ("Testing murmur3...",
              murmur::murmur_str, "./reference/murmur3_x64_128.tests");
}


