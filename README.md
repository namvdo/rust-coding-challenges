# Rust Coding Challenges

## Project Overview
The current problem while learning practical SE skills for students (including myself) is that there are too many tutorials on the internet for simple and common tasks, but when searching for things that are the bedrock to build functional applications, we're often stopped at some high-level explanation. These can give us the false impression that we understand, but later find that we're stuck on the first line of code. It's always better to get something done rather than doing nothing at all, that's why this project was created to practice CS fundamentals and SE skills that are universally essential regardless of the programming languages we're using.

### How will we do it?
* Real and practical problems when building software will be introduced as challenges in the project with different levels of complexity
* For each challenge:
  * A challenge folder with descriptions, input, output, and sample data, and the source and test folders
  * Inside the source folder, there will be the `solve()` and some other functions
  * README also includes relevant CS/SE concepts and the explanations for steps needed to complete the challenge
* After implementation, the result will be checked with the pre-defined test cases.
* Once all test cases are passed, your solution can be merged into the master!

#### What will we learn?
- Data structures
- Algorithms
- Parsing techniques
- Memory management
- Concurrency
- And more!

## Challenges
| Challenge Name  | Complexity | Author       | Relevant Knowledge                     |
|------------------|------------|--------------|----------------------------------------|
| [Simple Parser](https://github.com/namvdo/rust-coding-challenges/tree/master/challenges/simple-parser)     | Easy       | [Rudi Cilibrasi](https://github.com/rudi-cilibrasi)    | String manipulation, data structures    |

## Getting Started

To get started with the project, clone the repository and navigate to the desired challenge:

```bash
git clone <repository-url>
cd rust-coding-challenges/challenges
cargo run --bin <challenge_name>
```
## Contributing
#### We welcome contributions! If you would like to add a new challenge or improve existing ones, please follow these steps:

1.	Fork the repository.
2.	Create a new branch for your feature or fix.
3.	Use the create_challenge.sh script to set up your new challenge. This script will help you generate the necessary directory structure, create a new Cargo project, and set up a README.md file for your challenge.
4.	Make your changes.
5.	Submit a pull request detailing your changes.

#### Using `create_challenge.sh`

The create_challenge.sh script is designed to streamline the process of adding new challenges. You can run it as follows:
```bash
./scripts/create_challenge.sh <challenge_name> <author_name>
```
Replace <challenge_name> with the name of your challenge and <author_name> with your name. This will create a new challenge folder with the required structure.
