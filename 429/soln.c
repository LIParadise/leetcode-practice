#include <stdlib.h>
#include "nodeDefine.h"

typedef struct myNode{
   int data;
   struct myNode* prevPtr;
}myNode;

typedef struct myList{
   // stack-like structure
   // supports `pushBack`, `freeList`, and `popBack`
   int size;
   myNode* tail;
}myList;

void initList( myList* listPtr ){
   listPtr -> size = 0;
   listPtr -> tail = NULL;
}

void pushBack( myList* listPtr, int d ){
   myNode* n = malloc( sizeof(myNode) );
   n -> data = d;
   n -> prevPtr = listPtr -> tail;
   listPtr -> tail = n;
   ++ listPtr -> size;
}

int popBack( myList* listPtr ){
   int ret = listPtr -> tail -> data;
   myNode* tmp = listPtr -> tail;
   -- listPtr -> size;
   listPtr -> tail = listPtr -> tail -> prevPtr;
   free( tmp );
   return ret;
}

void freeList( myList* listPtr ){
   int s = listPtr -> size;
   while( s != 0 ){
      -- s;
      popBack ( listPtr );
   }
   initList ( listPtr );
}

int heightHelper( struct Node* tnptr ){
   // return tree height; modify (*sizePtr) such that it would have tree size;
   if( tnptr == NULL ){
      return 0;
   }
   int subTreeHeight = 0;
   int s = tnptr->numChildren;
   for( int j = 0; j<s; ++j ){
      int tmp = heightHelper( tnptr->children[j] );
      subTreeHeight = (tmp>subTreeHeight)? tmp : subTreeHeight;
   }
   ++subTreeHeight;
   return subTreeHeight;
}

void traverse( struct Node* tnptr, myList* listArr, int currentDepth ){
   // traverse the tree, and put the nodes we met into the array of linked lists
   // assume root have `currentDepth` being 0;
   // so we just put the nodes into the list `listArr[currentDepth]`

   if( tnptr == NULL ){
      return;
   }
   pushBack( listArr + currentDepth, tnptr -> val );
   int s = tnptr -> numChildren;
   for( int i = 0; i < s; ++i ){
      traverse( tnptr->children[i], listArr, currentDepth+1 );
   }
}

/**
 * Return an array of arrays of size *returnSize.
 * The sizes of the arrays are returned as *returnColumnSizes array.
 * Note: Both returned array and *columnSizes array must be malloced, assume caller calls free().
 */
int** levelOrder(struct Node* root, int* returnSize, int** returnColumnSizes){
   int height = 0;
   height = heightHelper( root );

   myList* listArr = malloc( sizeof(*listArr) * height );
   for( int i = 0; i < height; ++i ){
      initList ( listArr+i );
   }
   *returnSize = height;
   *returnColumnSizes = malloc( sizeof(**returnColumnSizes) * height );
   int** ret = malloc( sizeof(*ret) * height );

   traverse( root, listArr, 0 );
   for( int i = 0; i < height; ++i ){
      int s = listArr[i].size;
      (*returnColumnSizes)[i] = s;
      ret[i] = malloc( sizeof(*(ret[i])) * s );
      for( int j = s; j > 0; --j ){
         ret[i][j-1] = popBack( listArr + i );
      }
      freeList( listArr + i );
   }
   free(listArr);

   return ret;
}
