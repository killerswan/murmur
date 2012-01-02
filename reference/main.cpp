// Kevin Cantu
// build a list of test vectors
// (first for amd64, 128 bit version)

#include "MurmurHash3.h"
#include <stdio.h>

int main () {
   uint8_t fox[] = "The quick brown fox jumps over the lazy dog.";
   uint8_t foxlen = 44;

   uint32_t fox_x86_32[1];
   uint64_t fox_x86_64[2];
   uint64_t fox_x64[2];

   MurmurHash3_x86_32 (fox, foxlen, 0, fox_x86_32);
   MurmurHash3_x86_128 (fox, foxlen, 0, fox_x86_64);
   MurmurHash3_x64_128 (fox, foxlen, 0, fox_x64);

   //printf( "mm3 x86 32: %08X\n",    fox_x86_32[0] );
   //printf( "mm3 x86 64: %016lX %016lX\n", fox_x86_64[0], fox_x86_64[1] );
   printf( "%s, %016lX%016lX\n", fox, fox_x64[0], fox_x64[1] );
}


