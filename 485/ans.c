int findMaxConsecutiveOnes(int* nums, int numsSize) {

  int size = 0;
  int temp = 0;
  for( int i = 0; i < numsSize; i++ ){
    if( nums[i] == 0 ){
      if( temp > size ){
        size = temp;
      }
      temp = 0;
    }else{
      ++temp;
    }
  }
  if( temp > size ){
    size = temp;
  }

  return size;
      
}
