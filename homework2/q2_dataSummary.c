/*
CPSC 4100 / SPR18 / Craig Tanis
Homework 2
Question 2
author: Sam Rogers
*/

typedef unsigned int uint; // convenience alias for loops

#include "stdio.h"
#include "math.h"
int main(void) {
  // declare working variables
  int count;
  float min;
  float max;
  float average;
  float rms;
  float squareAverage;
  float sum = 0;
  float squareAccumulator = 0;

  // get input
  printf("Enter number of values: ");
  scanf("%d", &count);
  printf("Enter %d values: ", count);
  
  for(uint i = 0; i < count; i++) {
    float curVal;
    scanf("%f", &curVal);
    if(curVal < min || ! min) min = curVal;
    if(curVal > max || ! max) max = curVal;
    sum += curVal;
    squareAccumulator += curVal * curVal;
  }
  
  // summarize
  average = sum / count;
  squareAverage = squareAccumulator / count;
  rms = sqrt(squareAverage);
  
  // display summary of data
  printf("Minimum: %f\n", min);
  printf("Maximum: %f\n", max);
  printf("Average: %f\n", average);
  printf("RMS: %f", rms);
  
  // exit with success code
  return 0;
}
