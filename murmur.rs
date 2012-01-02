// Kevin Cantu
// replace djb with murmur


use std;


fn djb(&&ss: str) -> uint {
   // default str::hash
   // https://github.com/graydon/rust/blob/master/src/libcore/str.rs at ll. 43-50
   let uu: uint = 5381u;
   for cc: u8 in ss { uu *= 33u; uu += cc as uint; }
   ret uu;
}


// translate to hex
fn murmur_str(&&ss: str) -> str {
   let mm = murmur(ss);
   ret #fmt("%016x%016x", mm[0], mm[1]);
}


// murmur3 x64 128-bit
fn murmur(&&key_: str) -> [u64] {

   let key = str::bytes (key_);
   let data = key;
   let len = vec::len(key);
   let nblocks = len / 16u;

   // TODO: random seeds
   let seed = 0u64;
   let hh = [seed,seed];

   let cc = [ 0x_87c37b91114253d5_u64,
              0x_4cf5ad432745937f_u64 ];

   
   // rotation left
   fn rotl64 (x: u64, r: u64) -> u64 {
      ret (x << r) | (x >> (64u-r));
   }


   // conversion from 8 to 64 bits
   pure fn isEight (bytes: [u8]) -> bool {
      ret vec::len(bytes) == 8u;
   }

   fn convert_eight_u8_to_one_u64 (bb: [u8]) -> u64 {
      check isEight(bb);
      ret vec::foldl(0u64, bb, {|w, b| (w << 8u) + (b as u64)});
   }

   #[test]
   #[should_fail]
   fn test1_conversion_u8to64 () {
      let XXXX = convert_u8to64 ([1u8,2u8]);
   }

   #[test]
   fn test2_conversion_u8to64 () {
      let aa = [255u8,0u8,8u8,0u8, 20u8,0u8,0u8,1u8];
      std::io::println(#fmt("converted: %016x", convert_eight_u8_to_one_u64 (aa)));
   }

   fn convert_u8to64 (bb: [u8]) -> [u64] {
      fn lesser <copy TT> (aa: TT, bb: TT) -> TT {
         if aa < bb { aa } else { bb }
      }

      // windowing
      fn windowed <copy TT> (nn: uint, xs: [TT]) -> [[TT]] {
         let ws: [[TT]] = [];
         vec::iteri (xs, {|ii, x|
            let len: uint = vec::len(xs);
            if ii+(nn) < len {
               let w: [TT] = vec::slice ( xs, ii, ii+nn );
               vec::push (ws, w);
            }
         });
         ret ws;
      }

      // split into vector^2
      fn splitEvery <copy TT> (nn: uint, xs: [TT]) -> [[TT]] {
         let ys: [[TT]] = [];
         vec::iteri (xs, {|ii, x|
            let len = vec::len(xs);
            if ii % nn == 0u && ii < len {
               let y = vec::slice (xs, ii, lesser(nn+ii, len));
               vec::push (ys, y);
            }
         });
         ret ys;
      }

      // PENDING A
      let bbs = splitEvery(8u, bb);
      ret vec::map ( bbs, {|xs|
         convert_eight_u8_to_one_u64(xs)
      });
   }
   

   // fmix
   fn fmix (k_: u64) -> u64 {
      let kk = k_;

      kk ^= kk >> 33u;
      kk *= 0x_ff51afd7ed558ccd_u64;
      kk ^= kk >> 33u;
      kk *= 0x_c4ceb9fe1a85ec53_u64;
      kk ^= kk >> 33u;

      ret kk;
   }

   // PENDING B
   // what the original does is process the full size blocks as if they're u64
   // then takes the leftovers and twiddle those bytes
   // so i need to change things
   let blocks = convert_u8to64 ([0u8,0u8,0u8,0u8, 0u8,0u8,0u8,0u8, 0u8,0u8,0u8,0u8, 0u8,0u8,0u8,0u8]); 
   



   // TODO: truncate to u64 for djb replacement
   ret [1u64, 1u64];
}


