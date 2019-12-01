void _myReverseStr( char* , int, int  );

char* reverseStr( char* s, int k){
  int size = 0;
  int middle = (k/2);
  for( int i = 0; ; i ++ ){
    if( s[i] == 0 ){
      size = i;
      // we don't want to move the '\0' at the end of c-string.
      break;
    }
  }

  for( int i = 0; i < size; i += (k*2 ) ){
    if( ( i+k ) < size ){
      _myReverseStr( s, i, k );
    }else{
      _myReverseStr( s, i, (size-i));
    }
  }

  return s;
}


// helper functions

void _myReverseStr( char* s, int idx, int k ){

  // just reverse k characters from including idx.
  int middle = k/2;
  char temp = 0;
  for( int i = 0; i < middle; i++ ){
    temp = s[idx+i];
    s[idx+i] = s[idx+k-i-1];
    s[idx+k-i-1] = temp;
  }
}
