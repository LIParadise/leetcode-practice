/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */

typedef struct TreeNode {
   int val;
   struct TreeNode *left;
   struct TreeNode *right;
}TreeNode;

/**
 * Return an array of arrays of size *returnSize.
 * The sizes of the arrays are returned as *returnColumnSizes array.
 * Note: Both returned array and *columnSizes array must be malloced, assume caller calls free().
 */

/*
   Since the root (including any sub-tree) shall always occupy an odd index,
   and that the result must be equal in its left subtree array size and its right counterpart,
   both the total number of rows and total number of columns are fully determined by
   the tree height.

   Traverse the tree two times would do.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int heightHelper( struct TreeNode* tnptr, int* sizeP ){
   int tmpHeightRight = 1;
   int tmpHeightLeft  = 1;
   if( tnptr->left != NULL ){
      tmpHeightLeft = heightHelper( tnptr->left, sizeP ) + 1;
   }
   if( tnptr->right != NULL ){
      tmpHeightRight = heightHelper( tnptr->right, sizeP ) + 1;
   }
   ++ (*sizeP);
   return ( tmpHeightRight>= tmpHeightLeft )? tmpHeightRight : tmpHeightLeft;
}

char** at( char*** arr, int row, int col ){
   return (*(arr+row*col))+col;
}

void postOrderTraverse( struct TreeNode* subRoot, char*** arr, char* buf, int height, int currentHeight, int col, int subRootIdx ){
   // `buf` is a char[65], which is defined by the caller
   // let's hope it's enough...
   //
   // use `currentHeight` and `subRootIdx` to determine
   // where to put the string in the array `arr`
   // e.g.
   // suppose we want determine the string of the root of the whole tree,
   // of which height is 4:
   // .......7.......   ---  `currentHeight` is 4
   // ...3.......B...   ---  `currentHeight` is 3
   // .1...5...9...D.   ---  `currentHeight` is 2
   // *.*.*.*.*.*.*.*   ---  `currentHeight` is 1
   // then `subRootIdx` is 7, `currentHeight` is 4
   // and we shall call the subtree with `subRootIdx` being (7-2^(4-2) == 3) and (7+2^(4-2) == B), respectively
   // where both the callee would have `currentHeight` being (4-1) = 3
   //
   // notice arr[i][j] need to use `realloc`

   // we shall never further call deeper when `currentHeight` is 1
   // so we don't even check here.

   if( subRoot -> left != NULL ){
      postOrderTraverse(
            subRoot->left,
            arr,
            buf,
            height,
            currentHeight-1,
            col,
            subRootIdx-(1<<(currentHeight-2)) );
   }
   if( subRoot -> right != NULL ){
      postOrderTraverse(
            subRoot->right,
            arr,
            buf,
            height,
            currentHeight-1,
            col, 
            subRootIdx-(1<<(currentHeight-2)) );
   }
   memset( buf, '\0', 65*sizeof(char) );
   int l = sprintf( buf, "%d", subRoot -> val );

   *at(arr, height-currentHeight, subRootIdx) =
      realloc ( *at(arr, height-currentHeight, subRootIdx),
            l*sizeof( **at(arr, height-currentHeight, subRootIdx) ) );

   memcpy( arr[height-currentHeight][subRootIdx], buf, l+1 );
}

char *** printTree(struct TreeNode* root, int* returnSize, int** returnColumnSizes){
   int treeSize = 0;
   const int treeHeight = heightHelper ( root, &treeSize);
   char buf[65];
   const int columns = (1<<treeHeight)-1;
   /* suppose height is 4 (including node)
      then it will be like:
      ...3...
      .1...5.
    *.*.*.*
    that is, `returnSize` shall be 3,
    `returnColumnSizes` shall be 2^3-1,
    e.g.: height = 4:
    .......7.......
    ...3.......B...    returnSize: 4
    .1...5...9...D.    returnColumnSizes: 2^4-1 = 15
    *.*.*.*.*.*.*.*
    */

   returnSize = malloc( sizeof(*returnSize) * 1 );
   *returnSize = treeHeight;
   returnColumnSizes = malloc( sizeof(*returnColumnSizes) * treeHeight );
   for( int i = 0; i < treeHeight; ++i ){
      returnColumnSizes[i] = malloc( sizeof(**returnColumnSizes) );
      *(returnColumnSizes[i]) = columns;
   }

   char*** ret = malloc( sizeof(*ret) * (*returnSize) );
   for( int i = 0; i < (*returnSize); ++i ){
      ret[i] = malloc( sizeof(*ret[i]) * columns );
      for( int j = 0; j < columns; ++j ){
         ret[i][j] = calloc( sizeof(*ret[i][j]), 1 );
      }
   }

   postOrderTraverse( root, ret, buf, treeHeight, treeHeight, columns, columns/2 );

   return ret;
}
