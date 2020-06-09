// array have 3n+1 elements
// where all elements appears in three except for one
// find that element.
//
// try to implement it in linear time and constant memory

#include <string.h>

// \( \log_3{(2^8)} \simeq 5.05 < 6\)
// for every byte, it takes at most 6 ternary digits
// to fully represent the number.
#define ternaryIntLength (sizeof(int)*6)

typedef struct intInTernary {
   char data[ternaryIntLength];
}intInTernary;

void initIntInTernary ( intInTernary* ptr ){
   memset( ptr->data, 0, ternaryIntLength );
}

void toTernary( unsigned input, intInTernary* ptr ){
   // ptr is the buffer to put the ternary number;
   // assume 0 is MSB, (END-1) is LSB
   //
   // notice the operator `%` acts slightly unideal
   // when dealing with negative numbers,
   // so we use `unsigned` here.
   initIntInTernary ( ptr );
   int idx = ternaryIntLength;
   while( input != 0 ){
      -- idx;
      ptr->data[idx] = input%3;
      input = input/3;
   }
}

int ternaryToInt( intInTernary* ptr ){
   // since it's transformed from some int
   // it shall be still within range of int
   // provided that it's used in `singleNumber` function
   // that is, they cancel out.

   int i = 0;
   while( ptr->data[i] == '\0' && i < ternaryIntLength ){
      ++i;
   }

   size_t ret = ptr->data[i];
   while( i < ternaryIntLength-1 ){
      ++i;
      ret = ret*3;
      ret += ptr->data[i];
   }

   return ret;
}

void myTernaryAddition ( intInTernary* ret, intInTernary* p1, intInTernary* p2 ){
   // *ret = *p1 + *p2
   for( int i = 0; i < ternaryIntLength; ++i ){
      ret->data[i] = ( ((int)p1->data[i]) + ((int)p2->data[i]) )%3;
   }
}

int singleNumber(int* nums, int numsSize){
   intInTernary ret, tmp;
   initIntInTernary( &ret );
   initIntInTernary( &tmp );
   while( numsSize > 0){
      --numsSize;
      toTernary( nums[numsSize], &tmp );
      myTernaryAddition( &ret, &ret, &tmp );
   }
   return ternaryToInt( &ret );
}
