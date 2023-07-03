# Tic Tac Toe

<!-- ## Demo & Snippets

-   Include hosted link
-   Include images of app if CLI or Client App

--- -->

## Purpose
To create tic tac toe CLI game.
To practise and implement my knowledge of Rust.
For educational purposes I will be trying to implement as many techniques of rust as I can regardless if it is efficent or not.

---

<!-- ## Build Steps

-   how to build / run project
-   use proper code snippets if there are any commands to run

--- -->

## Design Goals / Approach

Recreation of tic tac toe in the cli

---

## Features

- 3x3 board
- User will be prompted to enter co ordinates, e.g. (A:2, B:3)
- Rematch and exit options
- Score Tracker

---

<!-- ## Known issues

-   Remaining bugs, things that have been left unfixed
-   Features that are buggy / flimsy

--- -->

## Future Goals

- Allow users to customise there names instead of player 1 and player 2.
- Make an Omok version, 19x19 board 5 in a row wins
- ~~Rematch, reset scores and exit options~~
- ~~Score tracker~~
- Testing

---

## Change logs

### 28/08/2023 | Project Creation
 Start of the project was a little bit bump, even though i read the documentation and followed along with the [lessons](https://doc.rust-lang.org/book/). I still had trouble figuring out the syntax, luckily the compiler was very helpful in telling me what I have to change inorder to make it compile. This just further solidifies the fact that I need to practise anything to fully understand the concepts, because just reading the theory and doing some code alongs dont cement the ideas properly.

 Other than syntax I have having a quite enjoyable time creating a rust program.

### 29/06/23023 | Practising using enums

 Initally I had been returning and using a bool value for the is_valid variable, but I wanted to practise using enums and the match operator. So I changed the placePiece function to return an enum which I feel allows me for more customisation when returning the function. Before I had only two options, but now I can have as many as I want allowing me to scale the program. 

### 01/07/2023 |  Check win function, changing from bool to enums

 Completed the check win function, but it feels a little bit too verbose. I wonder if there is a way for me to check it without creating several references to the board state. But other than it being too verbose, it works i just dont know how efficient it is.

 I have also changed all the bools that where checking for certain conditions to enum's for the same reasons as stated on the 29th.

### 03/07/2023 | Refactoed and debugged some functions, created new ones

- Debugged the is_all_true function, removed useless code from ther functions.
- Saved the scores on the board struct, created a method on the board struct to increment scores based on player victory.
- Allowed the user to exit and restart from the board or application
- Created a third loop within the main function to loop 

---

<!-- ## What did you struggle with?

-   What? Why? How?

--- -->