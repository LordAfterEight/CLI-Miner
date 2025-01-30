## **DON'T READ THIS IF YOU DON'T WANT THE GAME TO BE SPOILED FOR YOU**

> The following is a basic description of concepts I thought of that make the game unique and interesting.
> This does not mean they will definitely implemented, though I will try to do so.


This game starts out simple: You mine **Bits**, sell them, buy **Miners**, mine more **Bits**, eventually convert them and so on. You will be able to convert **Bits** to **Bytes** that eventually can be processed into **Data Strings** using a **Compressor**.

These **Data Strings** are a list of (8, 16, ...) **Bytes** containing random **Bits** (1's or 0's). They can be analyzed and then displayed as the according ASCII numbers, symbols or letters. An **Extractor** will extract the Letters that form **Code Blocks**. If there are more than one **Code Block** in a **Data String**, only the first one will be extracted.

**Code Blocks** are, as the name says, blocks of "code" that can be used to create custom scripts to automate certain actions such as mining, buying, selling, converting, etc. 

The whole thing could look like this:

    mined Bits: 64
    -> needed to convert to 8 Bytes
    Each Byte can be listed (Bits will be randomly generated): 
    
    Byte 1:  0 1 0 0 1 1 0 1
    Byte 2:  1 0 1 0 1 1 0 0
    Byte 3:  0 0 1 0 1 1 0 1
    Byte 4:  0 1 0 0 1 0 0 1
    Byte 5:  0 1 0 0 0 1 1 0
    Byte 6:  0 0 0 1 0 0 1 1
    Byte 7:  0 0 1 1 1 0 1 0
    Byte 8:  0 1 0 1 0 0 1 0

    Bytes converted to a single Data String (ASCII): M,-IF:R

    Extractor can extract "IF" from "M,-IF:R" because IF is a code block

Code Action Blocks that can be extracted from Data Strings:

    IF, GREATER, LESS, BUY, SELL
    
Code Variable Blocks that can always be used:

    Bits, Bytes, Miners, Converters, Extractors, Compressors


Everything here is subject to change
