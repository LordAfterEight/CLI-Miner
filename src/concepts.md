## **DON'T READ THIS IF YOU DON'T WANT THE GAME TO BE SPOILED FOR YOU**

> The following is a basic description of concepts I thought of that make the game unique and interesting.
> This does not mean they will definitely implemented, though I will try to do so.


## General

This game starts out simple: You mine **Bits**, sell them, buy **Miners**, mine more **Bits**, eventually convert them and so on. You will be able to convert **Bits** to **Bytes** that eventually can be processed into **Data Strings** using a **Compressor**.

These **Data Strings** are a list of (8, 16, ...) **Bytes** containing random **Bits** (1's or 0's). They can be analyzed and then displayed as the according ASCII numbers, symbols or letters. An **Extractor** will extract the Letters that form **Code Blocks**. If there are more than one **Code Block** in a **Data String**, only the first one will be extracted.

## Code Blocks

**Code Blocks** are, as the name says, blocks of "code" that can be used to create custom scripts to automate certain actions such as mining, buying, selling, converting, etc. 

The whole process from mining **Bits** to an extracted **Code Block** could look like this:

    mined Bits: 64
    -> needed to convert to 8 Bytes
    Each Byte can be listed (Bits will be randomly generated): 
    
    Byte 1:  0 1 0 0 1 1 0 1
    Byte 2:  0 0 1 0 1 1 0 0
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

## Deep Mechanics, based on Progress
At some point strange things will happen. The music you're used to will fade out, there will be complete silence. You will hear strange sounds, the music will change drastically. I won't be clear, it will sound weird, kind of muffled, yet familiar. Menus will behave weirdly, things will disappear, scripts will get corrupted. The only thing you can do is defend yourself and your game save before everything gets deleted. The only thing you can use to do so is the CLI, text and lines on a black screen. You will see that mining Bits, once all the game was about, is just a part of now necessary processes to progress, to defend yourself and to attack viruses. And maybe to expand to more than just a computer.

## Obviously...
...everything is subject to change. I am a beginner in programing using Rust, this is my first project using Rodio and Ratatui too. This is a very ambitious project that will take a lot of time and learning but I hope to implement everything that I wrote down here.
