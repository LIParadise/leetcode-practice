// array have 2n+1 elements
// where all elements appears in pair except for one
// find that element.
//
// try to implement it in linear time and constant memory

int singleNumber(int* nums, int numsSize){
   int ret = 0;
   while( numsSize > 0){
      --numsSize;
      ret = ret ^ nums[numsSize];
   }
   return ret;
}
