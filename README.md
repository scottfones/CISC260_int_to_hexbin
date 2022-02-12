# CISC 260: HW1 - Integer to 32-bit Hexadecimal and Binary

## Assignment Write-up

Write a program in the language of your choice (c ,c++, java, python, etc.) which takes as input an integer (positive or negative) in base 10, and returns a string representation in 32-bit of the number in hexadecimal and binary.

1. Use a twos-complement representation for negative numbers
2. You can create an array of symbols 0-F to make it easier to figure out each digit.
    1. char digits[]=[‘0’,’1’,’2’,’3’,’4’,’5’,’6’,’7’,’8’,’9’,’A’,’B’,’C’,’D’,’E’,’F’];
    2. then digits[12] will return ‘C’
3. You should convert the absolute value to binary first, then take the twos complement if the value is negative, then convert the binary to hexadecimal
4. You may not use any built in conversion operators or print operators that can do the conversion automatically (i.e. NO printf(‘%x’,number)).
