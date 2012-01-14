// Kevin Cantu
// hash function testing

use std;
use djb;
use bench;
use str2;

// main
fn main () {
   let meow = bench::word_of_god();

   bench::hash_bench ("Benching djb...     ", djb::djb,          meow);
//   bench::hash_bench ("Benching djb_...     ", djb::djb_,          meow);
}


