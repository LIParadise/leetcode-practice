#include "soln.c"
#include <stdio.h>

int main(){

   int arr0[] = {1,1,2,3};
   int arr1[] = {2,2,3,5,3,6};

   int dummy;
   int* dummyP = &dummy;
   
   int* p1 = singleNumber( arr0, 4, dummyP );
   int* p2 = singleNumber( arr1, 6, dummyP );

   printf( "%d %d\n", p1[0], p1[1] );
   printf( "%d %d\n", p2[0], p2[1] );

   free(p1);
   free(p2);

   return 0;
}
