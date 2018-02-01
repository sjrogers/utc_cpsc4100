/*
CPSC4100 / SPR18 / Prof. Craig Tanis
Homework 2
Question 1
author: Sam Rogers
*/

#include "stdio.h"
#include "stdlib.h"
#include "time.h"

typedef unsigned int uint; // convenience alias for loops
const uint ASCII_LOWERCASE_OFFSET = 32;

// header for random alphanumeric provider
char randAlphanum();

int main(void) {
  unsigned int height;
  unsigned int width;
  
  // seed pseudorandom number generator
  srand(time(NULL));
  
  printf("Enter height: ");
  scanf("%d", &height);
  printf("\n");
  printf("Enter width: ");
  scanf("%d", &width);
  printf("\nOutput (%dx%d):\n", height, width);
  
  // iterate for each row left to right
  for(uint row = 0; row < height; row++)
    {
      // iterate for each column in row
      for(uint col = 0; col < width; col++)
	{
	  char outChar = randAlphanum();
	  fputc(outChar, stdout);
	}
      // terminate row with newline
      fputc('\n', stdout);
    }
  
  // exit with success code
  return 0;
}

// implementation for random alphanumeric provider
char randAlphanum()
{
  /* function generates a pseudorandom alphanumeric character
    making use of even/odd properties of the seed, as well
    as the same properties of a random toggle value to make the
    output less obviously deterministic
  */

  // generate digit by default
  char startChar = '0';
  int modulator = 10;
  
  // get random numbers
  int seed = rand();
  int setFlipper = rand();
  
  // if toggle is even, generate an alpha character
  if(setFlipper % 2 == 0)
  {
    modulator = 26;
    startChar = 'A';
    // lowercase if toggle is twice an odd number
    if( (setFlipper / 2) % 2 == 1) startChar += ASCII_LOWERCASE_OFFSET;
  }
  
  return startChar + seed % modulator;
}
