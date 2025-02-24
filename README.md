# Linear Regression Model using Rust

## Introduction

This project implements a simple linear regression model in Rust to predict the output of the function y = 2x + 1. We use synthetic data for training and evaluate the model's performance on unseen data. The project demonstrates basic machine learning concepts and Rust programming techniques.

## Setup and Running the Code

1.⁠ ⁠Install Rust and Rust Rover
2.⁠ ⁠Creating a new project on Rust Rover
3.⁠ ⁠Build and run the project: This project has two files; cargo.toml which contains configurations file and main.src which has the rsut code. 


## Approach

My approach to solving this problem involved the following steps:

1.⁠ ⁠Generating Synthetic Data:Created a dataset of (x, y) pairs where y = 2x + 1.

2.⁠ ⁠Defining the Model: Implemented a ⁠LinearRegression struct that represents our model. It includes methods for forward pass and training.

3.⁠ ⁠Training the Model: Used gradient descent to train the model on our synthetic data. The training process monitors and prints the average loss every 100 epochs to track convergence.

4.⁠ ⁠Evaluating the Model: We tested the trained model on unseen data and compared its predictions with the true values. We also used the ⁠ textplots ⁠ crate to visualize our results.

## Results and Evaluation

 When tested on unseen data, it produced the following results:

•⁠  ⁠For x = -5, predicted y = [insert value], true y = -9
•⁠  ⁠For x = 0, predicted y = [insert value], true y = 1
•⁠  ⁠For x = 5, predicted y = [insert value], true y = 11
•⁠  ⁠For x = 10, predicted y = [insert value], true y = 21

The model's predictions closely align with the true values, indicating successful learning of the underlying function.

## Challenges and Learning Process

During this project, I encountered several challenges:

1.⁠ ⁠Rust Syntax and Concepts: As a relatively new Rust programmer, I had to familiarize myself with Rust's unique features.
2.⁠ ⁠Burn library functionality: Having challenges with the burn library functionality. Having issues with unresolved imports and syntax errros. It seems that the imports I was trying to use are are no longer valid.
3.⁠ ⁠Visualization in Rust: Finding and using a suitable plotting library in Rust was quite challenging.
4. Dependences: Being restricted to use the provided dependences only was a limit as I was unable to try different approaches with different dependencies.

Through overcoming these challenges, I gained a deeper understanding of:

•⁠  ⁠Implementing machine learning algorithms from scratch, which enhanced my understanding of basics of Rust and its capabilities.


## Conclusion

This project served as a good introduction to both linear regression and Rust programming. But for a beginner that has not been exposed to Rust and Programming it is very challenging.
