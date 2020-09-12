#include "ans.c"
#include <stdio.h>

TreeNode* create_tree( int* arr, int size){
   TreeNode* tree_arr[size];
   for( int i = 0; i < size; ++i ){ tree_arr[i] = NULL; }

   for( int i = 0; i < size; ++i ){
      if( arr[i] != -1 ){
         tree_arr[i] = malloc( sizeof(TreeNode) );
         tree_arr[i] -> val = arr[i];
         tree_arr[i] -> left = NULL;
         tree_arr[i] -> right = NULL;
         if( i%2 ){
            tree_arr[(i-1)/2] -> left  = tree_arr[i];
         }else{
            tree_arr[(i-1)/2] -> right = tree_arr[i];
         }
      }
   }

   return tree_arr[0];
}

void free_tree( TreeNode* root ){
   if( root -> right ) free_tree( root -> right );
   if( root -> left  ) free_tree( root -> left  );
   free(root);
}

int main(){

   int tree1_raw [11] = { 3,5,1,6,2,9,8,-1,-1,7,4 };
   int tree2_raw [15] = { 3,5,1,6,7,4,2,-1,-1,-1,-1,-1,-1,9,8};

   TreeNode* tree_1 = create_tree( tree1_raw, 11 );
   TreeNode* tree_2 = create_tree( tree2_raw, 15 );

   printf( "%d", (int)leafSimilar( tree_1, tree_2 ) );

   free_tree(tree_1);
   free_tree(tree_2);
}
