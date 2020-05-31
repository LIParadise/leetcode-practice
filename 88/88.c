// both ascending;
// the first have enough space to merge the array
// so we just merge the list from the end of the first array.
void merge(int* nums1, int nums1Size, int m, int* nums2, int nums2Size, int n){
  int cnt = m+n-1;
  --m;
  --n;
  for ( ; cnt >= 0; --cnt ){
    if ( m < 0 || n < 0 ){
      break;
    }else if ( m >= 0 && nums1[m] > nums2[n] ){
      nums1[cnt] = nums1[m];
      --m;
    }else if ( n >= 0 && !(nums1[m] > nums2[n]) ){
      nums1[cnt] = nums2[n];
      --n;
    }
  }

  // some left part
  // if "nums1" have something left, just ignore it
  // since they are already in the right location
  for ( ; n>=0 && cnt>=0; --cnt, --n ){
    nums1[cnt] = nums2[n];
  }
}
