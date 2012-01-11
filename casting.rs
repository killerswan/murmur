use std;
use vec2;

// OK this both compiles and works as expected!
//   BEFORE(aa): [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]
//   AFTER (dd): [0x04030201, 0x08070605]

// compare with libstd/io.rs at 77-84 (fn read())

#[test]
fn casting_test () {
   fn messy () unsafe {
      // casting u8 to u32
      fn u8to32 (aa: [u8]) -> [u32] unsafe
      {
         assert vec::len(aa) % 4u == 0u;
         let bb: *u8 = vec::unsafe::to_ptr(aa);
         let cc: *u32 = unsafe::reinterpret_cast(bb);
         let dd: [u32] = vec::unsafe::from_buf(cc, 2u);
         ret dd;
      }

      let aa: [u8] = [1u8, 2u8,3u8,4u8, 5u8,6u8,7u8,8u8];

      std::io::println("BEFORE(aa): " + vec2::show(aa, {|num| #fmt("0x%02x", num as uint)}));
      let dd = u8to32(aa);
      std::io::println("AFTER (dd): " + vec2::show(dd, {|num| #fmt("0x%08x", num as uint)}));
   }

   std::io::println("");
   messy();
}


