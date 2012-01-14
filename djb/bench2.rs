// Kevin Cantu
// hash function testing

#[link(name = "bench2", author = "kcantu", vers = "0.0")];

use std;
use str2;

fn hash_bench <TT> ( label: str, hashfn: native fn(&&str)->TT, data: [str] )
{
   let t0 = std::time::precise_time_s();
   let _v = vec::map(data, hashfn);
   let t1 = std::time::precise_time_s();

   std::io::println(label + #fmt("%06.3f sec", t1 - t0));
}


