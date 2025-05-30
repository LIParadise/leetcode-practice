// graph[i] is a list of indices that have connection w/ point "i"
// e.g. graph[3] == [1, 2, 4]
// then we have edge (1,3), (2,3), (3,4)
//
// no self-edge
// no parallel-edge
// graph is not directed


// a DFS pre-order traversal and coloring all the nodes shall do.
#include <stdbool.h>
#include <stdlib.h>
#ifdef   DEBUG
#include <assert.h>
#endif   // DEBUG

typedef enum myColor {
  NOCOLOR   = 0x0,  // 0b0000
  LIGHTRED  = 0x1,  // 0b0001
  DEEPRED   = 0x3,  // 0b0011
  LIGHTGREY = 0x4,  // 0b0100
  DARKGREY  = 0xc,  // 0b1100
  LIGHTMASK = 0x5,  // 0b0101
  REDMASK   = 0x1,  // 0b0001
  GREYMASK  = 0x4   // 0b0100
} myColor;

bool isLight ( myColor m ){
  return ( LIGHTMASK | m ) == LIGHTMASK;
}

bool isRed ( myColor m ){
  return ( REDMASK & m ) == REDMASK;
}

bool isGrey ( myColor m ){
  return ( GREYMASK & m ) == GREYMASK;
}

bool colorCollide ( myColor m1, myColor m2 ){
  return (m1 & m2) != NOCOLOR && m1 != NOCOLOR && m2 != NOCOLOR;
}

bool colorPair ( myColor m1, myColor m2 ){
  return (m1 & m2) == NOCOLOR && m1 != NOCOLOR && m2 != NOCOLOR;
}

void darken ( myColor* mp ){
  if ( *mp == LIGHTRED ) {
    *mp = DEEPRED;
  }else if ( *mp == LIGHTGREY ){
    *mp = DARKGREY;
  }
}

bool isBipartiteDfsCheck (int**, int, int*, myColor*, int );

bool isBipartite(int** graph, int graphSize, int* graphColSize){
  myColor* graphColor = (myColor*)calloc( graphSize, sizeof(myColor));
  for( int i = 0; i < graphSize; ++i ){
    graphColor[i] = NOCOLOR;
  }

  for( int i = 0; i < graphSize; ++i ){
    if ( graphColor[i] == NOCOLOR ) {
      graphColor[i] = LIGHTGREY;
      if ( !isBipartiteDfsCheck (graph, graphSize, graphColSize, graphColor, i ) ){
        return false;
      }
    }
  }
  free ( graphColor );
  return true;
}

bool isBipartiteDfsCheck (int** graph, int graphSize, int* graphColSize, myColor* graphColor, int id ) {

  myColor tmpColor;
  if ( !isLight ( graphColor[id] ) ){
    return true;
  }
  if ( isRed ( graphColor[id] ) ){
    tmpColor = LIGHTGREY;
  }else if ( isGrey ( graphColor[id] ) ){
    tmpColor = LIGHTRED;
  }else {
#ifdef   DEBUG
    assert( 0 && "color wrong" );
#endif
  }

  int i = 0;
  for( ; i < graphColSize[id]; ++i ){
    if ( colorCollide ( graphColor[id], graphColor[graph[id][i]] ) ){
      // color collsion, (grey to grey) or (red to red), it's not bipartite.
      return false;
    }
  }

  i = 0;
  for( ; i < graphColSize[id]; ++i ){
    if ( colorPair ( graphColor[id], graphColor[graph[id][i]] ) ){
      // somebody is checking, let'em do this for us.
      continue;
    }else {
      graphColor[graph[id][i]] = tmpColor;
      if( !isBipartiteDfsCheck ( graph, graphSize, graphColSize, graphColor, graph[id][i] ) ){
        return false;
      }
    }
  }
  darken( graphColor + id );
  return true;
}

#include <stdio.h>

int main() {
  int** graph = calloc ( 4, sizeof(int*) );
  int   arr0[3] = {1,2,3};
  int   arr1[2] = {0,2};
  int   arr2[3] = {0,1,3};
  int   arr3[2] = {0,2};
  graph[0] = arr0;
  graph[1] = arr1;
  graph[2] = arr2;
  graph[3] = arr3;
  int   arrs[4] = {3,2,3,2};
  int*  graphColSize = arrs;

  if (isBipartite(graph, 4, graphColSize) ){
    printf ( "success\n" );
    return 0;
  }

  printf ( "failure\n" );
  return 1;
}
