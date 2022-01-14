A wordle solver.

It relies on the [word list](https://www-cs-faculty.stanford.edu/~knuth/sgb-words.txt) from [stanford graph base](https://www-cs-faculty.stanford.edu/~knuth/sgb.html). If the file is not found, it is downloaded.

NB: It differs from how the wordle responds to duplicate letters.

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
