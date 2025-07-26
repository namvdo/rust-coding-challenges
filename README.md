# Rust Coding Challenges

## Project Overview
### Motivation
The current problem while learning practical SE skills for students (including myself) is that there are too many tutorials on the internet for basic and trivial tasks, but when searching for things that are the bedrock to build functional applications, we're often stopped at some high-level explanation. These can give us the false impression that we feel we know how to build software, but later find that we're stuck on the first line of code. Our lives indeed become easier and easier as technology evolves, programming languages are easier to learn due to the high level of abstraction, open-source software, and libraries can help us do most of our daily tasks, LLM and coding assistants are ubiquitous, search engines are now more likely give us all we need from the very first page, these factors are super conducive to converting a technical problem of our own into the business problem, which is eventually what we want to achieve. 

Knowledge without practicability is mostly useless, by practicing fundamental concepts in computer science through hands-on programming exercises, will help us to get the knots of what is "under" the hoods and test the applicability of what we think we know. And the fun part about this is that, when we actually write down something, that's the progress where new information comes up, and at the end, it's our ability to form a coherent picture of information that we've learned.

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
| [wc command](https://github.com/namvdo/rust-coding-challenges/tree/master/challenges/wc-command) | Easy |  [namvdo](https://github.com/namvdo) | MMap, unsafe operations, string manipulation, parallel processing | 

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
