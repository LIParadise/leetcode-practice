#include "soln.c"
#include <stdio.h>

int main (){

   TreeNode* root = malloc( sizeof(*root) );
   root -> val = 2;
   root -> left  = malloc( sizeof(*(root->left)) );
   root -> right = malloc( sizeof(*(root->right)) );

   root -> left  -> val   = 1;
   root -> left  -> left  = NULL;
   root -> left  -> right = NULL;
   root -> right -> val   = 4;
   root -> right -> left  = malloc( sizeof(*(root->right->left)) );
   root -> right -> right = malloc( sizeof(*(root->right->right)) );

   root -> right -> left  -> val   = 3;
   root -> right -> left  -> left  = NULL;
   root -> right -> left  -> right = NULL;
   root -> right -> right -> val   = 5;
   root -> right -> right -> left  = NULL;
   root -> right -> right -> right = NULL;

   int a = 0;
   int *b = malloc ( sizeof(*b) * 2 );

   char*** arr = printTree( root, &a, &b );

   for( int i = 0; i < a; ++i ){
      for( int j = 0; j < b[i]; ++j ){
         printf( "%s ", *at( arr, i, j) );
      }
      printf("\n");
   }

   for( int i = 0; i < a; ++i ){
      for( int j = 0; j < b[i]; ++j ){
         free( *at( arr, i, j) );
      }
   }
   free ( b);

   free ( root -> right -> right );
   free ( root -> right -> left );
   free ( root -> left );
   free ( root -> right );
   free ( root );

   return 0;
}
