#include <pthread.h>
#include <string.h>
#include <stdlib.h>
#include <stdio.h>

/**
 * Note: The returned array must be malloced, assume caller calls free().
 */

static const int singlePuzzleLen = 7;
static const char myTrue  = '\1';
static const char myFalse = '\0';

typedef struct myData 
{
  char ** words;
  char ** puzzles;
  int  *  retPtr;
  int     wordsSize;
  int     puzzlesIdx1;
  int     puzzlesIdx2;
} myData;

void init_myData ( myData* dataPtr, char ** words, 
    char ** puzzles, int * retPtr, int wordsSize , int puzzlesIdx1, int puzzlesIdx2 )
{
  dataPtr -> words       = words;
  dataPtr -> puzzles     = puzzles;
  dataPtr -> retPtr      = retPtr;
  dataPtr -> wordsSize   = wordsSize;
  dataPtr -> puzzlesIdx1 = puzzlesIdx1;
  dataPtr -> puzzlesIdx2 = puzzlesIdx2;
}

void* calc ( void* ptr )
{
  myData* dataPtr = (myData*)(ptr);
  int     puzzleIndicator  [26];
  int     wordIndicator    [26];
  int     puzzlesHeadIdx = 0;
  char    thisWordFailed = myFalse;

  for( int p = dataPtr->puzzlesIdx1; p < dataPtr->puzzlesIdx2; ++p ){

    /* init "puzzles[p]" */
    memset( puzzleIndicator, 0, 26*sizeof(int) );
    for( int i = 0; i < singlePuzzleLen ; ++i ){
      puzzleIndicator [  dataPtr->puzzles[p][i]-'a'  ] = 1;
    }
    puzzlesHeadIdx = dataPtr->puzzles[p][0]-'a';

    /* scan for "words[w]" */
    for( int w = 0; w < dataPtr->wordsSize; ++w ){

      thisWordFailed = myFalse;

      memset ( wordIndicator, 0, 26*sizeof(int) );
      for( int i = 0; dataPtr->words[w][i] != '\0'; ++i ){
        wordIndicator[ dataPtr->words[w][i]-'a' ] = 1;
      }

      if( wordIndicator[ puzzlesHeadIdx ] == 0 ){
        continue;
      }
      for ( int i = 0; i < 26; ++i ) {
        if( wordIndicator[i] == 1 && puzzleIndicator[i] == 0 ){
          thisWordFailed = true;
          break;
        }
      }
      if( thisWordFailed == false ){
        ++dataPtr->retPtr[p];
      }

    }

  }

  return (void*)NULL;
}

int* findNumOfValidWords(char ** words, int wordsSize, char ** puzzles, int puzzlesSize, int* returnSize)
{
  int* retPtr = (int*) malloc ( sizeof(int) * puzzlesSize );
  memset ( retPtr, 0, puzzlesSize*sizeof(int) );
  *returnSize = puzzlesSize;

  if( puzzlesSize == 1){

    myData d;
    init_myData ( &d, words, puzzles, retPtr, wordsSize, 0, puzzlesSize );
    calc( &d );

  } else if ( puzzlesSize <= 100 ) {

    myData d1, d2;
    pthread_t thrd_arr[2];
    thrd_arr[0] = 0;
    thrd_arr[1] = 1;
    void*     dummy_arr[2] = {0};
    init_myData ( &d1, words, puzzles, retPtr, wordsSize, 0            , puzzlesSize/2 );
    init_myData ( &d2, words, puzzles, retPtr, wordsSize, puzzlesSize/2, puzzlesSize   );
    pthread_create ( thrd_arr+0  , NULL, calc, ( (void*)(&d1) ) );
    pthread_create ( thrd_arr+1  , NULL, calc, ( (void*)(&d2) ) );
    pthread_join ( thrd_arr[0] , dummy_arr );
    pthread_join ( thrd_arr[1] , dummy_arr + 1);

  } else if ( puzzlesSize <= 200 ){
    myData d1, d2, d3, d4;
    pthread_t thrd_arr[4];
    thrd_arr[0] = 0;
    thrd_arr[1] = 1;
    thrd_arr[2] = 2;
    thrd_arr[3] = 3;
    void*     dummy_arr[4] = {0};
    init_myData ( &d1, words, puzzles, retPtr, wordsSize, 0               , puzzlesSize/4*1 );
    init_myData ( &d2, words, puzzles, retPtr, wordsSize, puzzlesSize/4*1 , puzzlesSize/4*2 );
    init_myData ( &d3, words, puzzles, retPtr, wordsSize, puzzlesSize/4*2 , puzzlesSize/4*3 );
    init_myData ( &d4, words, puzzles, retPtr, wordsSize, puzzlesSize/4*3 , puzzlesSize     );
    pthread_create ( thrd_arr+0  , NULL, calc, ( (void*)(&d1) ) );
    pthread_create ( thrd_arr+1  , NULL, calc, ( (void*)(&d2) ) );
    pthread_create ( thrd_arr+2  , NULL, calc, ( (void*)(&d3) ) );
    pthread_create ( thrd_arr+3  , NULL, calc, ( (void*)(&d4) ) );
    pthread_join ( thrd_arr[0] , dummy_arr );
    pthread_join ( thrd_arr[1] , dummy_arr + 1);
    pthread_join ( thrd_arr[2] , dummy_arr + 2);
    pthread_join ( thrd_arr[3] , dummy_arr + 3);
  }else {
    myData d1, d2, d3, d4, d5, d6, d7, d8;
    pthread_t thrd_arr[8]  = {0,1,2,3,4,5,6,7};
    void*     dummy_arr[8] = {0};
    init_myData ( &d1, words, puzzles, retPtr, wordsSize, 0               , puzzlesSize/8*1 );
    init_myData ( &d2, words, puzzles, retPtr, wordsSize, puzzlesSize/8*1 , puzzlesSize/8*2 );
    init_myData ( &d3, words, puzzles, retPtr, wordsSize, puzzlesSize/8*2 , puzzlesSize/8*3 );
    init_myData ( &d4, words, puzzles, retPtr, wordsSize, puzzlesSize/8*3 , puzzlesSize/2   );
    init_myData ( &d5, words, puzzles, retPtr, wordsSize, puzzlesSize/2   , puzzlesSize/8*5 );
    init_myData ( &d6, words, puzzles, retPtr, wordsSize, puzzlesSize/8*5 , puzzlesSize/8*6 );
    init_myData ( &d7, words, puzzles, retPtr, wordsSize, puzzlesSize/8*6 , puzzlesSize/8*7 );
    init_myData ( &d8, words, puzzles, retPtr, wordsSize, puzzlesSize/8*7 , puzzlesSize     );
    pthread_create ( thrd_arr+0  , NULL, calc, ( (void*)(&d1) ) );
    pthread_create ( thrd_arr+1  , NULL, calc, ( (void*)(&d2) ) );
    pthread_create ( thrd_arr+2  , NULL, calc, ( (void*)(&d3) ) );
    pthread_create ( thrd_arr+3  , NULL, calc, ( (void*)(&d4) ) );
    pthread_create ( thrd_arr+4  , NULL, calc, ( (void*)(&d5) ) );
    pthread_create ( thrd_arr+5  , NULL, calc, ( (void*)(&d6) ) );
    pthread_create ( thrd_arr+6  , NULL, calc, ( (void*)(&d7) ) );
    pthread_create ( thrd_arr+7  , NULL, calc, ( (void*)(&d8) ) );
    pthread_join ( thrd_arr[0] , dummy_arr );
    pthread_join ( thrd_arr[1] , dummy_arr + 1);
    pthread_join ( thrd_arr[2] , dummy_arr + 2);
    pthread_join ( thrd_arr[3] , dummy_arr + 3);
    pthread_join ( thrd_arr[4] , dummy_arr + 4);
    pthread_join ( thrd_arr[5] , dummy_arr + 5);
    pthread_join ( thrd_arr[6] , dummy_arr + 6);
    pthread_join ( thrd_arr[7] , dummy_arr + 7);
  }

  return retPtr;
}

int main ()
{
  char * words [7] = {0};
  for( int i = 0; i < 7; ++i ){
    words [i] = malloc ( 10*sizeof(char) );
    memset ( words[i], 0, 10*sizeof(char) );
  }
  memcpy ( words[0], "aaaa"   , 5 );
  memcpy ( words[1], "asas"   , 5 );
  memcpy ( words[2], "able"   , 5 );
  memcpy ( words[3], "ability", 8 );
  memcpy ( words[4], "actt"   , 5 );
  memcpy ( words[5], "actor"  , 6 );
  memcpy ( words[6], "access" , 7 );

  char * puzzles [6] = {0};
  for( int i = 0; i < 6; ++i ){
    puzzles [i] = malloc ( 8*sizeof(char) );
    memset ( puzzles[i], 0, 8*sizeof(char) );
  }
  memcpy ( puzzles[0], "aboveyz", 8 );
  memcpy ( puzzles[1], "abrodyz", 8 );
  memcpy ( puzzles[2], "abslute", 8 );
  memcpy ( puzzles[3], "absoryz", 8 );
  memcpy ( puzzles[4], "actresz", 8 );
  memcpy ( puzzles[5], "gaswxyz", 8 );

  int returnSize = 0;
  int wordsSize  = 7;
  int puzzlesSize = 6;
  int* returned_value = findNumOfValidWords (
      words, wordsSize, puzzles, puzzlesSize, &returnSize
      );

  printf ( "results: %d, %d, %d, %d, %d, %d\n",
      returned_value[0], returned_value[1], returned_value[2], returned_value[3], returned_value[4], returned_value[5] );

  free( returned_value );
  for( int i = 0; i < 7; ++i ){
    free ( words[i] );
  }
  for( int i = 0; i < 6; ++i ){
    free ( puzzles[i] );
  }
  return 0;

}
