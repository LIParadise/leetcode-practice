// array have 3n+1 elements
// where all elements appears in three except for one
// find that element.
//
// try to implement it in linear time and constant memory

#include <string.h>

int singleNumber(int* nums, int numsSize){
   unsigned tmp;
   unsigned ret = 0;
   for( int i = 0; i < sizeof(int)*8; ++i ){
      tmp = 0;
      for( int j = 0; j < numsSize; ++j ){
         if( 1 & (nums[j] >> i) ){
            ++ tmp;
         }
      }
      tmp = tmp%3;
      if( tmp ){
         ret |= tmp<<i;
      }
   }

   return ret;
}
