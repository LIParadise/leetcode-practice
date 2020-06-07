#include "soln.c"
#include <stdio.h>

int main (){

   TreeNode* root = malloc( sizeof(*root) );
   root -> val = 1;
   root -> left = malloc( sizeof(*(root->left)) );
   root -> right = NULL;

   root -> left -> val = 2;
   root -> left -> left = NULL;
   root -> left -> right = NULL;

   int* a;
   int** ar;

   char*** arr = printTree( root, a, ar );

   for( int i = 0; i < *a; ++i ){
      for( int j = 0; j < *(ar[i]); ++j ){
         printf( "%s ", *at( arr, *a, *(ar[i]) ) );
      }
      printf("\n");
   }

   for( int i = 0; i < *a; ++i ){
      for( int j = 0; j < *(ar[i]); ++j ){
         free ( *at( arr, *a, *(ar[i]) )  );
      }
      free( ar[i] );
   }
   free(a);

   free ( root -> left );
   free ( root );

   return 0;
}
