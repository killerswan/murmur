// Kevin Cantu
// replace djb with murmur

use std;
use vec2;

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
   ret #fmt("%016x%016x", mm[0u], mm[1u]);
}

// murmur3 x64 128-bit
fn murmur(&&key_: str) -> [u64] {

   let key = str::bytes (key_);

   // TODO: random seeds
   let seed = 0u64;
   let h1 = seed;
   let h2 = seed;

   let c1 = 0x_87c37b91114253d5_u64;
   let c2 = 0x_4cf5ad432745937f_u64;

   
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
      let _XXXX = convert_u8to64 ([1u8,2u8]);
   }

   #[test]
   fn test2_conversion_u8to64 () {
      let aa = [255u8,0u8,8u8,0u8, 20u8,0u8,0u8,1u8];
      std::io::println(#fmt("converted: %016x", convert_eight_u8_to_one_u64 (aa)));
      let test2 = convert_u8to64 (aa+aa);
   }

   fn convert_u8to64 (bb: [u8]) -> [u64] {
      fn lesser <copy TT> (aa: TT, bb: TT) -> TT {
         if aa < bb { aa } else { bb }
      }

      // split into vector^2
      fn splitEvery <copy TT> (nn: uint, xs: [TT]) -> [[TT]] {
         let ys: [[TT]] = [];

         vec::iteri (xs, {|ii, _x|
            let len = vec::len(xs);

            if ii % nn == 0u && ii < len {
               let y = vec::slice (xs, ii, lesser(nn+ii, len));
               vec::push (ys, y);
            }
         });

         ret ys;
      }

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


   // split body data and tail
   let nbytes  = vec::len(key);
   let nblocks = nbytes / 16u;
   let (blocks_, tail) = vec2::splitAt (nblocks*16u, key);
   let blocks: [u64] = convert_u8to64 (blocks_);


   // crunch body data
   let ii = 0u;
   assert nblocks % 2u == 0u;
   while (ii < nblocks) {

      let k1 = blocks[ii*2u+0u];
      let k2 = blocks[ii*2u+1u];

      k1 *= c1;
      k1 = rotl64(k1, 31u64);
      k1 *= c2;
      h1 ^= k1;

      h1 = rotl64(h1, 27u64);
      h1 += h2;
      h1 = h1*5u64 + 0x_52dce729_u64;

      k2 *= c2;
      k2 = rotl64(k2, 33u64);
      k2 *= c1;
      h2 ^= k2;

      h2 = rotl64(h2, 31u64);
      h2 += h1;
      h2 = h2*5u64 + 0x_38495ab5_u64;


      ii += 1u;
   }


   // tail
   vec::grow(tail, 16u - vec::len(tail), 0u8);
   assert vec::len(tail) == 16u;
   let jj = vec::reversed (tail);
   let jj_ = convert_u8to64 (jj);
   assert vec::len(jj_) == 2u;
   let j1 = jj_[1u]; // reversing
   let j2 = jj_[0u]; // reversing

   j2 *= c2;
   j2 = rotl64(j2, 33u);
   j2 *= c1;
   h2 ^= j2;

   j1 *= c1;
   j1 = rotl64(j1, 31u);
   j1 *= c2;
   h1 ^= j1;
   

   // finalization

   h1 ^= nbytes;
   h2 ^= nbytes;

   h1 += h2;
   h2 += h1;
   
   h1 = fmix(h1);
   h2 = fmix(h2);

   h1 += h2;
   h2 += h1;
   

   ret [h1, h2];
   //ret test2;
}


