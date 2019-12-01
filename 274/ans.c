#include <stdlib.h>  // qsort();
int _myCmpInt( int* a, int* b ){
  return (*a)-(*b);
}
int hIndex(int* citations, int citationsSize) {
  if( citationsSize == 0 ){
    return 0;
  }
  qsort( citations, citationsSize, sizeof(int), _myCmpInt );
  int i = 0;
  for( ; i < citationsSize; i++ ){
    if( citationsSize-i <= citations[i] ){
      // there are ( citationsSize - i ) papers that have citations >= citations[i];
      return ( ( citationsSize - i ) < citations [i] )? ( citationsSize-i) : (citations[i]) ;
    }
  }
  return 0;
}
