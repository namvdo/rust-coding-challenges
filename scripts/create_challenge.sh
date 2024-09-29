#!/bin/bash

if [ -z "$1" ]; then
  echo "Usage: ./create_challenge.sh <challenge_name>"
  exit 1
fi
ROOT_CHALLENGE_NAME=rust-coding-challenges
CHALLENGE_NAME=$1
AUTHOR_NAME=$2
CHALLENGE_DIR="challenges/$CHALLENGE_NAME"

# Create a new Cargo project for the challenge
cargo new --lib "$CHALLENGE_DIR"

# Add the rust-coding-challenges dependency

CHALLENGE_CARGO_TOML=$CHALLENGE_DIR/Cargo.toml
echo "Adding root project dependency to $CHALLENGE_CARGO_TOML"
sed -i '' '/\[dependencies\]/a \
rust-coding-challenges = { path = "../../" }' "$CHALLENGE_CARGO_TOML"

cat <<EOT > "$CHALLENGE_DIR/src/main.rs"
use rust_coding_challenges::utils::read_text_file_from_args;
use $CHALLEGE_NAME::solve;
fn main() -> std::io::Result<()> {
    let content = read_text_file_from_args()?;
    let result = solve(&content);
    
    // Print results or perform other actions as needed
    Ok(())
}
EOT

# Create the lib.rs file with a sample solve function
cat <<EOL > "$CHALLENGE_DIR/src/lib.rs"
pub fn solve(input: &str) -> Result<(), String> {
    // TODO: Implement the solution for $CHALLENGE_NAME
    Ok(())
}
EOL

# Create the tests directory and test.rs file with a sample test case
# Create the test file for the challenge
mkdir -p "$CHALLENGE_DIR/tests/inputs"

cat <<EOL > "$CHALLENGE_DIR/tests/test.rs"
#[cfg(test)]
mod tests {
    use super::*;
    use $CHALLENGE_NAME::solve;

    #[test]
    fn test_solve() {
        // Call the solve function and check for expected results
        let result = solve();
        assert!(result.is_ok());
        // Add more assertions based on expected behavior
    }
}
EOL


# Create the README.md file
README_PATH="$CHALLENGE_DIR/README.md"
echo "# $CHALLENGE_NAME" > "$README_PATH"
echo "" >> "$README_PATH"
echo "## Given By $AUTHOR_NAME" >> "$README_PATH"
echo "" >> "$README_PATH"
echo "## Topics" >> "$README_PATH"
echo "" >> "$README_PATH"
echo "## Main Functions" >> "$README_PATH"
echo "" >> "$README_PATH"
echo "## Example Output" >> "$README_PATH"
echo "" >> "$README_PATH"
echo "## Usage Example" >> "$README_PATH"
echo "" >> "$README_PATH"

echo "Challenge '$CHALLENGE_NAME' created successfully!"