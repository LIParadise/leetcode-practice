// "name" is a string, containing the genuine name of someone
// "typed" is another string, which may be generated from typing
// "name" with some repetitive key strokes
//
// return if this is the case.
// e.g. "name" == "dio saama" --> "typed" could be "diooo saammaa"
// but "diooo samma" shall return false.

#include <stdbool.h>

bool isLongPressedName(char * name, char * typed){
  int n = 0, t = 0;

  while( name[n] == typed[t] && name[n] != '\0' && typed[t] != '\0' ){

    if( name[n+1] == name[n] || ( name[n+1] != name[n] && typed[t+1] != typed[t] ) ){
      // wlog we could assume all repetitive strokes happen in the last
      // character of which appears consecutively
      // e.g.
      // "dio saama"
      //       *!
      // the asterik marked 'a', which is "name[5]", shall NOT repeat.
      // only the exclamation mark 'a', which is "name[6]", could repeat.
      ++n; ++t;
      if( typed[t] == '\0' || name[n] == '\0' ){
        break;
      }else{
        continue;
      }
    }else { // name[n+1] != name[n], consecutive strokes

      // just increment the pointer of "typed"
      ++t; continue;
    }
  }

  return ( name[n] == '\0' && typed[t] == '\0' );

}

#include <stdio.h>

int main( int argc, char** argv ){
  if( argc != 3 ){
    printf( "RTFM\n" );
    return 1;
  }
  
  char* output = (
    (isLongPressedName(argv[1], argv[2]))?
    "success" :
    "fail" );

  printf ( "%s", output );

  return 0;
}
