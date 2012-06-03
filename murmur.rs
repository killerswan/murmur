// Kevin Cantu
// replace djb with murmur

#[link(name = "murmur", author = "kcantu", vers = "0.2")];

use std;

// rotation left
fn rotl64(x: u64, r: u64) -> u64 
{
   ret (x << r) | (x >> (64u-r));
}

// fmix
fn fmix(k_: u64) -> u64 
{
   let mut kk = k_;

   kk ^= kk >> 33u;
   kk *= 0x_ff51afd7ed558ccd_u64;
   kk ^= kk >> 33u;
   kk *= 0x_c4ceb9fe1a85ec53_u64;
   kk ^= kk >> 33u;

   ret kk;
}

fn apply_constants(rot: u64, 
                   kk: u64,
                   h: u64, 
                   c: u64,
                   c_: u64) -> u64 
{

   let mut k = kk;
   k *= c;
   k  = rotl64(k, rot);
   k *= c_;
   ret h ^ k;
}

// crunch body data
#[inline(always)]
fn do_body(blocks: [u64],
           nblocks: uint,
           h1_: u64,
           h2_: u64,
           c1: u64,
           c2: u64) -> (u64,u64) 
{
   let mut h1 = h1_;
   let mut h2 = h2_;

   let mut ii = 0u;

   //std::io::println(#fmt("nblocks: %u", nblocks));

   while ii < nblocks
   {
      let k1 = blocks[ii*2u + 0u];
      let k2 = blocks[ii*2u + 1u];

      h1 = apply_constants(31u64, k1, h1, c1, c2);

      h1 = rotl64(h1, 27u64);
      h1 += h2;
      h1 = h1*5u64 + 0x_52dce729_u64;

      h2 = apply_constants(33u64, k2, h2, c2, c1);

      h2 = rotl64(h2, 31u64);
      h2 += h1;
      h2 = h2*5u64 + 0x_38495ab5_u64;

      ii += 1u;
   }

   ret (h1, h2);
}

// tail
fn do_tail(tail: [u8], h1: u64, h2: u64, c1: u64, c2: u64) -> (u64,u64) { 
   let len = vec::len(tail);

   let mut k1 = 0u64;
   let mut k2 = 0u64;

   if 15u <= len { k2 ^= (tail[14u] as u64) << 48u}
   if 14u <= len { k2 ^= (tail[13u] as u64) << 40u}
   if 13u <= len { k2 ^= (tail[12u] as u64) << 32u}
   if 12u <= len { k2 ^= (tail[11u] as u64) << 24u}
   if 11u <= len { k2 ^= (tail[10u] as u64) << 16u}
   if 10u <= len { k2 ^= (tail[ 9u] as u64) <<  8u}
   if  9u <= len { k2 ^= (tail[ 8u] as u64) <<  0u}
   if  8u <= len { k1 ^= (tail[ 7u] as u64) << 56u}
   if  7u <= len { k1 ^= (tail[ 6u] as u64) << 48u}
   if  6u <= len { k1 ^= (tail[ 5u] as u64) << 40u}
   if  5u <= len { k1 ^= (tail[ 4u] as u64) << 32u}
   if  4u <= len { k1 ^= (tail[ 3u] as u64) << 24u}
   if  3u <= len { k1 ^= (tail[ 2u] as u64) << 16u}
   if  2u <= len { k1 ^= (tail[ 1u] as u64) <<  8u}
   if  1u <= len { k1 ^= (tail[ 0u] as u64) <<  0u}

   let h2_ = apply_constants(33u64, k2, h2, c2, c1);
   let h1_ = apply_constants(31u64, k1, h1, c1, c2);
   ret (h1_,h2_);
}

// finalization
fn finalize(h1_: u64, h2_: u64, len: uint) -> (u64,u64) 
{
   let mut h1 = h1_;
   let mut h2 = h2_;

   h1 ^= len as u64;
   h2 ^= len as u64;

   h1 += h2;
   h2 += h1;

   h1 = fmix(h1);
   h2 = fmix(h2);

   h1 += h2;
   h2 += h1;

   ret (h1, h2);
}

// I suspect this flips the endianness from the C/C++ equivalent
fn to_u64(bb: [u8]) -> ([u64], [u8]) unsafe {
   let len = vec::len(bb);
   let head_len = (len / 8u) * 8u;

   let mut head = vec::slice(bb, 0u, head_len);
   let mut tail = vec::slice(bb, head_len, len);

   let mut vv: [u64] = ::unsafe::reinterpret_cast(head);
   ::unsafe::forget(head);
   vec::unsafe::set_len(vv, vec::len(bb) / 8u);

   ret (vv, tail);
}

#[test]
fn TEST_0_to_u64() {
   let (xx, yy) = to_u64([1u8,2u8,3u8,4u8,5u8,6u8,7u8,8u8,9u8,10u8]);
   assert [0x08070605_04030201_u64] == xx;
   assert [9u8, 10u8] == yy;
}

// murmur3 x64 128-bit
fn murmur(&&key_: str) -> [u64] 
{

   let key = str::bytes(key_);

   // TODO: random seeds
   let seed = 0u64;
   let mut h1 = seed;
   let mut h2 = seed;

   let c1 = 0x_87c37b91114253d5_u64;
   let c2 = 0x_4cf5ad432745937f_u64;

   // split body data and tail
   let nbytes  = vec::len(key);
   let nblocks = nbytes / 16u;
   let body_ = vec::slice(key, 0u, nblocks*16u);
   let tail = vec::slice(key, nblocks*16u, nbytes);
   let (body, _) = to_u64(body_);

   let (h1, h2) = do_body(body, nblocks, h1, h2, c1, c2);
   let (h1, h2) = do_tail(tail, h1, h2, c1, c2);
   let (h1, h2) = finalize(h1, h2, nbytes);

   ret [h1, h2];
}

// translate to hex
fn murmur_str(&&ss: str) -> str 
{
   let mm = murmur(ss);
   ret #fmt("%016X%016X", mm[0u], mm[1u]);
}

fn murmur_chopped(&&ss: str) -> u64
{
   ret murmur(ss)[0];
}


