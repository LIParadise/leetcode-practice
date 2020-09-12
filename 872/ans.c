/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */

struct TreeNode {
	int val;
   struct TreeNode *left;
   struct TreeNode *right;
};

typedef char bool;

#include <stddef.h>
#include <stdlib.h>
#include <string.h>

#ifdef DEBUG
#  include <stdio.h>
#endif // DEBUG

typedef struct TreeNode TreeNode;

static const size_t MAX_TREE_SIZE = 200;

void dump_tree_content( TreeNode*, int**, int* );

void dump_tree( TreeNode* tree_ptr, int** buf ){
   *buf = realloc( (*buf), MAX_TREE_SIZE*sizeof(int) );
   for( unsigned u = 0; u < MAX_TREE_SIZE; ++u ){ (*buf)[u] = -1; }
   int idx = 0;
   dump_tree_content( tree_ptr, buf, &idx );
}

void dump_tree_content( TreeNode* tree_ptr, int** buf, int* idx ){
   if( tree_ptr -> left ) {
      dump_tree_content( tree_ptr -> left, buf, idx );
   }
   if( tree_ptr -> left == NULL && tree_ptr -> right == NULL ){
      (*buf)[(*idx)] = tree_ptr -> val;
      ++(*idx);
   }
   if( tree_ptr -> right ) {
      dump_tree_content( tree_ptr -> right, buf, idx );
   }
}

bool leafSimilar(struct TreeNode* root1, struct TreeNode* root2){
   int *tree_2_content = NULL, *tree_1_content = NULL;
   dump_tree( root1, &tree_1_content );
   dump_tree( root2, &tree_2_content );

   unsigned short idx1 = 0, idx2 = 0;
   bool flag = 1;
   for( unsigned u = 0; u < MAX_TREE_SIZE; ++u ){
      if( tree_1_content[u] != tree_2_content[u] ){
         flag = 0;
         break;
      }
   }

#ifdef DEBUG
   for( unsigned u = 0; u < 20; ++u ){ printf( "%d ", tree_1_content[u] ); }
   printf( "%c\n", (char)8 );
   for( unsigned u = 0; u < 20; ++u ){ printf( "%d ", tree_2_content[u] ); }
   printf( "%c\n", (char)8 );
#endif // DEBUG

   free( tree_1_content );
   free( tree_2_content );

   return flag;
}
