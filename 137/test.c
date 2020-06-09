#include <stdio.h>
#include "soln.c"

int main() {
   int  arr[4] = {1,2,1,1};
   int  arr1[7] = {3,2147483648,2147483648,-2147483647,2147483648,3,3};
   int  arr2[25] = {43,16,45,89,45,-2147483648,45,2147483646,-2147483647,-2147483648,43,2147483647,-2147483646,-2147483648,89,-2147483646,89,-2147483646,-2147483647,2147483646,-2147483647,16,16,2147483646,43};
   int  arr3[4] = {-400,-400,200,-400};
   int  arr4[10] = {-2,-2,1,1,-3,1,-3,-3,-4,-2};


   printf( "shall be 2, it's %d\n", singleNumber( arr, 4 ) );
   printf( "shall be -2147483647, it's %d\n", singleNumber( arr1, 7 ) );
   printf( "shall be 2147483647, it's %d\n", singleNumber( arr2, 25 ));
   printf( "shall be 200, it's %d\n", singleNumber( arr3, 4 ) );
   printf( "shall be -4, it's %d\n", singleNumber( arr4, 10 ) );

   printf ( "\n\n" );
   
   intInTernary test;
   toTernary ( 2147483647, &test );
   printf ( "%d\n", ternaryToInt ( &test ) );
   return 0;

}
