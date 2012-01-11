// Kevin Cantu
// additional vector functions

#[link(name = "vec2", author = "kcantu", vers = "0.0")];
use std;
use str2;

fn windowed <TT: copy> (nn: uint, xx: [TT]) -> [[TT]] {
   let ww = [];

   assert 1u <= nn;

   vec::iteri (xx, {|ii, _x|
      let len = vec::len(xx);

      if ii+nn <= len {
         let w = vec::slice ( xx, ii, ii+nn );
         vec::push (ww, w);
      }
   });

   ret ww;
}

#[test]
fn windowed_test_3of6 () {
   assert [[1u,2u,3u],[2u,3u,4u],[3u,4u,5u],[4u,5u,6u]]
          == windowed (3u, [1u,2u,3u,4u,5u,6u]);
}

#[test]
fn windowed_test_4of6 () {
   assert [[1u,2u,3u,4u],[2u,3u,4u,5u],[3u,4u,5u,6u]]
          == windowed (4u, [1u,2u,3u,4u,5u,6u]);
}

#[test]
#[should_fail]
fn windowed_test_0of6 () {
   let _x = windowed (0u, [1u,2u,3u,4u,5u,6u]);
}

#[test]
fn windowed_test_7of6 () {
   assert [] == windowed (7u, [1u,2u,3u,4u,5u,6u]);
}

fn splitAt <TT: copy> (nn: uint, xx: [TT]) -> ([TT],[TT]) {
   assert 0u <= nn;
   assert nn <= vec::len(xx);
   (vec::slice(xx,0u,nn), vec::slice(xx,nn,vec::len(xx)))
}

#[test]
fn splitAt_test_a () {
   let temp: ([u8],[u8]) = splitAt(0u, []);
   assert ([],[]) == temp;
}

#[test]
fn splitAt_test_b () {
   assert ([],[1u]) == splitAt(0u, [1u]);
}

#[test]
fn splitAt_test_c () {
   assert ([1u],[]) == splitAt(1u, [1u]);
}

#[test]
#[should_fail]
fn splitAt_test_d () {
   let temp: ([u8],[u8]) = splitAt(7u, []);
   assert ([],[]) == temp;
}


fn take <TT: copy> (nn: uint, xx: [TT]) -> [TT] {
   assert nn < vec::len(xx);
   ret vec::slice (xx, 0u, nn);
}

#[test]
fn take_test_a () {
   assert [] == take(0u, [8u,9u]);
}

#[test]
#[should_fail]
fn take_test_b () {
   let _xyz = take(6u, [8u,9u]);
}

// given a function to stringify an element
// make a pretty string of a vector
fn show <TT> ( vv: [TT],
                    showTT: block(TT)->str ) -> str {

   let strings = vec::map (vv, showTT);


   ret "[" + str::connect(strings, ", ") + "]";
}

#[test]
fn show_test_a () {
   assert "[3, 56, 62, 2, 4]"
         == show ([3u,56u,62u,2u,4u], {|x| #fmt("%u", x)});
}

#[test]
fn show_test_b () {
   let vv   =  [[ "abc",  "def",  "ghi" ],[ "jkl",  "mno" ]];
   let ref  = "[[\"abc\", \"def\", \"ghi\"], [\"jkl\", \"mno\"]]";

   // this is a bit fragile, but works ok for now...
   let test = show (vv, {|xs|
                 show (xs, {|x| "\"" + x + "\"" })
              });

   assert ref == test;
}

   
// system dependent
fn u8to32 (aa: [u8]) -> [u32] unsafe {
   assert vec::len(aa) % 4u == 0u;
   let len = vec::len(aa) / 4u;

   let bb: *u8 = vec::unsafe::to_ptr(aa);
   let cc: *u32 = unsafe::reinterpret_cast(bb);
   let dd: [u32] = vec::unsafe::from_buf(cc, len);
   ret dd;
}

// system dependent
fn u8to64 (aa: [u8]) -> [u64] unsafe {
   assert vec::len(aa) % 8u == 0u;
   let len = vec::len(aa) / 8u;

   let bb: *u8 = vec::unsafe::to_ptr(aa);
   let cc: *u64 = unsafe::reinterpret_cast(bb);
   let dd: [u64] = vec::unsafe::from_buf(cc, len);
   ret dd;
}


#[test]
fn amd64_casting_test_32 () {
   let aa: [u8] = [1u8, 2u8,3u8,4u8, 5u8,6u8,7u8,8u8];
   let bb = u8to32(aa);

   assert [0x_04030201_u32, 0x_08070605_u32] == bb;
}

#[test]
fn amd64_casting_test_64 () {
   let aa: [u8] = [1u8, 2u8,3u8,4u8, 5u8,6u8,7u8,8u8];
   let bb: [u64] = u8to64(aa);

   //std::io::println(#fmt("0x_%016x", bb[0]));
   assert [0x_08070605_04030201_u64] == bb;
}

#[test]
fn amd64_casting_test_64_nil () {
   let aa: [u8] = [];
   let bb: [u64] = u8to64(aa);

   assert [] == bb;
}

#[test]
fn amd64_casting_test_32_nil () {
   let aa: [u8] = [];
   let bb: [u32] = u8to32(aa);

   assert [] == bb;
}

#[test]
#[should_fail]
fn amd64_casting_test_64_too_long () {
   let aa: [u8] = [1u8,2u8,3u8,4u8, 5u8,6u8,7u8,8u8, 9u8];
   let _bb: [u64] = u8to64(aa);
}

#[test]
#[should_fail]
fn amd64_casting_test_64_too_short () {
   let aa: [u8] = [1u8,2u8,3u8,4u8, 5u8,6u8,7u8 ];
   let _bb: [u64] = u8to64(aa);
}

#[test]
#[should_fail]
fn amd64_casting_test_32_too_long () {
   let aa: [u8] = [1u8,2u8,3u8,4u8, 5u8,6u8,7u8,8u8, 9u8];
   let _bb: [u32] = u8to32(aa);
}

#[test]
#[should_fail]
fn amd64_casting_test_32_too_short () {
   let aa: [u8] = [1u8,2u8,3u8,4u8, 5u8,6u8,7u8 ];
   let _bb: [u32] = u8to32(aa);
}




