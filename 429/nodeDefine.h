#ifndef _NODEDEFINE_H_
#define _NODEDEFINE_H_

struct Node {
   int val;
   int numChildren;
   struct Node** children;
}Node;

#endif // _NODEDEFINE_H_
