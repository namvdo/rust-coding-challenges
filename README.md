# Rust Coding Challenges

## Project Overview

TBU

- Data structures
- Algorithms
- Parsing techniques
- Memory management
- Concurrency
- And more!

Each challenge will be contained within its own subdirectory and will include the problem statement, solution, relevant background knowledge, and testing data.

## Challenges
| Challenge Name  | Complexity | Author       | Relevant Knowledge                     |
|------------------|------------|--------------|----------------------------------------|
| Simple Parser     | Easy       | [Rudi Cilibrasi](https://github.com/rudi-cilibrasi)    | String manipulation, data structures    |

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
