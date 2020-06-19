/**
 * Note: The returned array must be malloced, assume caller calls free().
 */
#ifndef _SOLN__
#define _SOLN__

#include <stdlib.h>

int* singleNumber(int* nums, int numsSize, int* returnSize){
   int tmp0 = 0, tmp1;
   unsigned bitMask = 1;
   int* ret = malloc(2 * sizeof(*ret));
   *returnSize = 2;
   for( int i = 0; i < numsSize; ++i ){
      tmp0 = tmp0^nums[i];
   }

   while( (bitMask & tmp0) == 0 ){
      bitMask = bitMask << 1;
   }
   
   tmp0 = 0;
   tmp1 = 0;
   for( int i = 0; i < numsSize; ++i ){
      if( bitMask & nums[i] ){
         tmp0 = tmp0 ^ nums[i];
      }else{
         tmp1 = tmp1 ^ nums[i];
      }
   }

   ret[0] = tmp0;
   ret[1] = tmp1;

   return ret;
}

#endif // _SOLN__
