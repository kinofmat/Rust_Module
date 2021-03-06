# Overview

I wrote a simple program to create a reciept from an order from a resturaunt. It has stored some items from a menu, and prices that the user can choose from.It will then total together the cost of all of the items, and display the tax on the items. Then it will prompt the user to select the tip amount, and loop through again if there is another order.

Having never used or had exposure to Rust before, this was to build a basic understanding of Rust, and get to use some unique features of the language.  

[Software Demo Video](https://youtu.be/IBC8SQhJS60)

# Development Environment

Visual Studio Code, with Rust and CodeLLDB extentions. Additionally had to install Rust, and C++ for Visual Studio. Used Cargo for initially creating the project as well. I took the HashMap, and io from the standard library, but had to specifically import it. 

# Useful Websites

* [Rust - Tutorials Point](https://www.tutorialspoint.com/rust/index.htm)
* [Rust Book](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html)
* [Rust By Example](https://doc.rust-lang.org/rust-by-example/index.html)

# Future Work

* Create input checking for the other option on tips.
* Pull the menu items from a db instead of having them coded in.
* Allow for items to be added or removed from the menu as well.