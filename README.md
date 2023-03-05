# rust-new-project-template
A good starting point for a new Rust project

## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)

Happ number judgment.

This program checks whether a given number is a happy number or not. A happy number is a number which eventually reaches 1 when replaced by the sum 
of the square of each digit.

The program first defines two functions, sum_of_squares and is_happy_number. The sum_of_squares function takes an integer as input and returns the 
sum of squares of its digits. The is_happy_number function takes an integer as input, and uses a HashSet to keep track of previously seen numbers, 
while repeatedly calling sum_of_squares on the input number until either it reaches 1 (which means the number is happy), or the number has already 
been seen before (which means the number is not happy).


In the main function, the program prompts the user to input a number, reads the input from the user, and calls the is_happy_number function to 
determine whether the number is happy or not. If the number is happy, the program prints a message saying so, and if it's not happy, the program 
prints a message indicating that it's not.


Overall, this program is a simple implementation of the algorithm to determine whether a number is a happy number or not, and demonstrates the use 
of basic Rust constructs such as functions and collections.

For example:

<img width="577" alt="Week 7 Rust mini project" src="https://user-images.githubusercontent.com/70717089/222941015-ad0ffbf7-9372-4550-815a-90a0f49de7e0.png">
