A wordle solver.

It relies on the [word list](https://www-cs-faculty.stanford.edu/~knuth/sgb-words.txt) from [stanford graph base](https://www-cs-faculty.stanford.edu/~knuth/sgb.html). If the file is not found, it is downloaded.

To solve wordle
===============
Just run the executable without any command line arguments.

For instance, `cargo run`. The program responds with a word. 
Enter the wordle reponse in the form of five letter string comprising of Y,N or G -- Y for Yellow, G for Green and N for nothing.



To play wordle
===============
Dont. Play at [wordle](https://www.powerlanguage.co.uk/wordle). However, if you want to practice, run it with argument `wordle`.
The program responds with a five letter string consisting of Y,N or G. 

* Y at position i => the letter at ith position in your guess is yellow.
* G at position i => the letter at ith position in your guess is green.
* N at position i => the letter at ith position in your guess is not present in the word.


To play cows and bulls
=======================
Run it with the argument `cb`. You will have upto 20 chances to figure out the right word.


Other Command Line arguments
=============================

* `soft`. By passing soft as argument, it will solve wordle in non-hard mode. It uses upto four words to figure out as many letters as possible and use the remaining chances to build up on it. This is akin to [Norvig's solution]() although, the choice of words is different.

* `shapes target-word response*` By giving a target word and a bunch of options, it will try and provide a sequence of words (without repition) that will satisfy the pattern. 
   +  For example, running with `shapes brine YNNNY YNNNY NYYYN YNNNY YNNNY` would give five distinct words which result in a H shape (when the result is known to be BRINE).
  
   ![image](https://user-images.githubusercontent.com/113322/156870121-cadfef10-95e1-4c0b-bf8a-cfd0f9fa2813.png)


   + running with `shapes brine YNNNY NYNYN NNYNN NYNYN YNNNY` would result in five distinct words that would result in an X shape (when the result is known to be BRINE).
   
   ![image](https://user-images.githubusercontent.com/113322/156870095-c73f629b-7cd6-4508-8af0-ae91266ead0a.png)

   
