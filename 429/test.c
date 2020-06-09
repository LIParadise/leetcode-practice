#include "soln.c"
#include <stdio.h>

void freeNode ( struct Node* np ){
   int s = np -> numChildren;
   while( s > 0 ){
      --s;
      freeNode ( np->children[s] );
   }
   free(np->children);
   free(np);
}

int main (){

   typedef struct Node Node;

   Node* root = malloc( sizeof(*root) );
   root -> val = 2;
   root -> numChildren = 2;
   root -> children = malloc( sizeof(*(root -> children)) * 2 );
   root -> children[0] = malloc(sizeof(Node));
   root -> children[1] = malloc(sizeof(Node));

   root -> children[0] -> val = 1;
   root -> children[0] -> numChildren = 0;
   root -> children[0] -> children = NULL;
   root -> children[1] -> val = 4;
   root -> children[1] -> numChildren = 2;
   root -> children[1] -> children = malloc( sizeof(*(root -> children)) * 2 );
   root -> children[1] -> children[0] = malloc(sizeof(Node));
   root -> children[1] -> children[1] = malloc(sizeof(Node));

   root -> children[1] -> children[0] -> val = 3;
   root -> children[1] -> children[0] -> numChildren = 0;
   root -> children[1] -> children[0] -> children = NULL;
   root -> children[1] -> children[1] -> val = 5;
   root -> children[1] -> children[1] -> numChildren = 0;
   root -> children[1] -> children[1] -> children = NULL;

   int a = 0;
   int *b = NULL;

   int** arr = levelOrder( root, &a, &b );

   for( int i = 0; i < a; ++i ){
      printf ( "[" );
      for( int j = 0; j < b[i]; ++j ){
         printf( "%d, ", arr[i][j] );
      }
      printf("\b\b]\n");
   }

   for( int i = 0; i < a; ++i ){
      free( arr[i] );
   }
   free( arr);
   free ( b);

   freeNode( root );

   return 0;
}
