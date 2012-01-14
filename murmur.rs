// Kevin Cantu
// replace djb with murmur

#[link(name = "murmur", author = "kcantu", vers = "0.1")];

use std;
use vec2;

// rotation left
fn rotl64 (x: u64, r: u64) -> u64 
{
   ret (x << r) | (x >> (64u-r));
}

// fmix
fn fmix (k_: u64) -> u64 
{
   let kk = k_;

   kk ^= kk >> 33u;
   kk *= 0x_ff51afd7ed558ccd_u64;
   kk ^= kk >> 33u;
   kk *= 0x_c4ceb9fe1a85ec53_u64;
   kk ^= kk >> 33u;

   ret kk;
}

fn apply_constants (rot: u64, 
                    kk: u64,
                    h: u64, 
                    c: u64,
                    c_: u64) -> u64 
{

   let k = kk;
   k *= c;
   k  = rotl64(k, rot);
   k *= c_;
   ret h ^ k;
}

// crunch body data
fn do_body (blocks: [u64],
            nblocks: uint,
            h1_: u64,
            h2_: u64,
            c1: u64,
            c2: u64) -> (u64,u64) 
{
   let h1 = h1_;
   let h2 = h2_;

   let ii = 0u;

   //std::io::println(#fmt("nblocks: %u", nblocks));

   while (ii < nblocks) 
   {
      let k1 = blocks[ii*2u + 0u];
      let k2 = blocks[ii*2u + 1u];

      h1 = apply_constants (31u64, k1, h1, c1, c2);

      h1 = rotl64(h1, 27u64);
      h1 += h2;
      h1 = h1*5u64 + 0x_52dce729_u64;

      h2 = apply_constants (33u64, k2, h2, c2, c1);

      h2 = rotl64(h2, 31u64);
      h2 += h1;
      h2 = h2*5u64 + 0x_38495ab5_u64;

      ii += 1u;
   }

   ret (h1, h2);
}

// tail
fn do_tail (tail: [u8], h1: u64, h2: u64, c1: u64, c2: u64) -> (u64,u64) { 
   let len = vec::len(tail);

   let k1 = 0u64;
   let k2 = 0u64;

   fn if_ (b: bool, f: block()) {
      if b { f() }
      else { () }
   }

      if_ (15u <= len, {|| k2 ^= (tail[14u] as u64) << 48u});
      if_ (14u <= len, {|| k2 ^= (tail[13u] as u64) << 40u});
      if_ (13u <= len, {|| k2 ^= (tail[12u] as u64) << 32u});
      if_ (12u <= len, {|| k2 ^= (tail[11u] as u64) << 24u});
      if_ (11u <= len, {|| k2 ^= (tail[10u] as u64) << 16u});
      if_ (10u <= len, {|| k2 ^= (tail[ 9u] as u64) <<  8u});
      if_ ( 9u <= len, {|| k2 ^= (tail[ 8u] as u64) <<  0u});
      if_ ( 8u <= len, {|| k1 ^= (tail[ 7u] as u64) << 56u});
      if_ ( 7u <= len, {|| k1 ^= (tail[ 6u] as u64) << 48u});
      if_ ( 6u <= len, {|| k1 ^= (tail[ 5u] as u64) << 40u});
      if_ ( 5u <= len, {|| k1 ^= (tail[ 4u] as u64) << 32u});
      if_ ( 4u <= len, {|| k1 ^= (tail[ 3u] as u64) << 24u});
      if_ ( 3u <= len, {|| k1 ^= (tail[ 2u] as u64) << 16u});
      if_ ( 2u <= len, {|| k1 ^= (tail[ 1u] as u64) <<  8u});
      if_ ( 1u <= len, {|| k1 ^= (tail[ 0u] as u64) <<  0u});

   let h2_ = apply_constants (33u64, k2, h2, c2, c1);
   let h1_ = apply_constants (31u64, k1, h1, c1, c2);
   ret (h1_,h2_);
}

// finalization
fn finalize (h1_: u64, h2_: u64, len: uint) -> (u64,u64) 
{
   let h1 = h1_;
   let h2 = h2_;

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

// murmur3 x64 128-bit
fn murmur(&&key_: str) -> [u64] 
{

   let key = str::bytes (key_);
   //std::io::println("key: "  + vec2::show(key,  {|b| #fmt("%02X", b as uint)}));

   // TODO: random seeds
   let seed = 0u64;
   let h1 = seed;
   let h2 = seed;

   let c1 = 0x_87c37b91114253d5_u64;
   let c2 = 0x_4cf5ad432745937f_u64;

   // split body data and tail
   let nbytes  = vec::len(key);
   let nblocks = nbytes / 16u;
   let (body_, tail) = vec2::splitAt (nblocks*16u, key);
   let body: [u64] = vec2::u8to64 (body_);

   let (h1, h2) = do_body (body, nblocks, h1, h2, c1, c2);
   let (h1, h2) = do_tail (tail, h1, h2, c1, c2);
   let (h1, h2) = finalize (h1, h2, nbytes);

   ret [h1, h2];
}


// translate to hex
fn murmur_str(&&ss: str) -> str 
{
   let mm = murmur(ss);
   ret #fmt("%016X%016X", mm[0u], mm[1u]);
}


