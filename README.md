# Authors:
## Amir Rahmat & Eason Chen

# Acknowledgement:
## Help from collaborative work, included pair programming, sharing and reading documentation of crates, and discussing ideas for implementation such as functions, library changes, etc. We also took ideas from our professor's array2, to fix the original implementation of array2 in our custom library.

# Implementation:
## We believe the library is correctly implemented, as well as the rotation functions. These were the most time consuming parts of the project. And we believe that the test cases were instrumental in developing functionality in the beginning. We however didn't implement other transformations like 270 rotation, flip, and transpose.

# Architecture:
## We parse through arguments, determine file name, rotation type, and iterator type. (argument processing)
## Functions for 90 and 180 degree rotations, are built off iterators, constructors, and structs from our custom array2 library, and utilizes Rgb type from the CSC411 image crate. 
## Main function handles the passing of data/information, it creates an array with Rgb image pixels, turns it into a 2d array using our custom library, then passes it into the appropriate function call, depending on rotation, and a parameter defining the type of iteration. The specific rotation function then processes the image, in either row or col major.
## A destination rgb image struct from the CSC411 image crate, is then initialized, and the write function write all the data from the vector holding the returned values from the rotation functions.

# Measurements:
## __________|row-major|col-major|
## 180 degree| 50.99ms | 59.89ms |
##  90 degree| 52.80ms | 57.35ms |

## Our measurements did not completely match predictions from Part B, we believe col-major may be longer than row-major because row major has more locality and col major has less locality. 180 row major has the fastest performance as predicted, because, it has the highest hit rate as the image is stored by row major order. This means that we will have high locality when accessing the rows. When mapping rows to rows, we will experience the same locality properties for the destination image processing. 90 degree row major is the second fastest, as predicted because the rows are stored in row major, so accessing it in row major results in high locality, but since we are mapping row to column we experience less locality then 180. For 180 col major it is the slowest as predicted, because accessing in column major results in low locality, and mapping column to column it is even lower locality. For our prediction we thought that 90 degree row and col major would be tied, but 90 degree column major is the third fastest, because while we are accessing in column major, it won't be as slow as 180 col major, because we are mapping rows to column.

# Potentially Better Memory Layouts
## We would have a 2d array that would have a constructor in either row-major or col-major order, so if your rotating an image 90 degrees from a 2d array in row major order, and you are iterating through in row major order, you can write to a destination 2d array, that is in column major order. So as you write in 90 degrees, you write the rows to the columns, so since the destination is in column major order, it has high locality, and as your reading the original 2d array in row major order you also have high locality. 


# Hours Spent
## About 10 hours of focused work

