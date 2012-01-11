// Kevin Cantu
// additional string functions I've wanted in Rust

/* so far including:
   splitfn
   lines
   words
*/

#[link(name = "str2", author = "kcantu", vers = "0.0")];
use std;

// a more general split
// unicode safe
// using a function, e.g., char::is_whitespace instead of single u8
fn splitfn(ss: str, sepf: block(cc: char)->bool) -> [str] {
   let vv: [str] = [];
   let accum: str = "";
   let ends_with_sep: bool = false;

   str::iter_chars(ss, {|cc|
      if sepf(cc) {
         vv += [accum];
         accum = "";
         ends_with_sep = true;
      } else {
         str::push_char(accum, cc);
         ends_with_sep = false;
      }
   });
   if str::char_len(accum) != 0u || ends_with_sep {
      vv += [accum];
   }
   ret vv;
}

#[test]
fn test_splitfn_a () {
   let data = "ประเทศไทย中华Việt Nam";
   assert ["ประเทศไทย中", 
           "Việt Nam"]
      == splitfn (data, {|cc| cc == '华'});
}

fn lines (ss: str) -> [str] {
   ret splitfn(ss, {|cc| cc == '\n'});
}

#[test]
fn test_lines_splitfn () {
   let data = "\nMary had a little lamb\nLittle lamb\nLittle lamb\n";

   assert ["", "Mary had a little lamb", "Little lamb", "Little lamb", ""]
      == lines(data);
}

fn words (ss: str) -> [str] {
   ret vec::filter( splitfn(ss, {|cc| char::is_whitespace(cc)}), 
                    {|w| 0u < str::char_len(w)});
}

#[test]
fn test_words_splitfn () {
   let data = "\nMary had a little lamb\nLittle lamb\nLittle lamb\n";

   assert ["Mary","had","a","little","lamb","Little","lamb","Little","lamb"]
      == words(data);
}

#[test]
fn test_example () {
   // TODO: correct documentation for str::char_range_at
   let s = "Clam chowder, hot sauce, pork rinds";
   let i = 0u;
   while i < str::char_len(s) {

     let {ch, next} = str::char_range_at(s, i);
     // TODO: why can't I use _ch in this record assignment to silence warnings?

     //log(debug, ch);
     //std::io::println(#fmt("%c",ch));
     i = next;
   }
}


