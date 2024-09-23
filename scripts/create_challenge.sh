#!/bin/bash

if [ -z "$1" ]; then
  echo "Usage: ./create_challenge.sh <challenge_name>"
  exit 1
fi

CHALLENGE_NAME=$1
AUTHOR_NAME=$2
CHALLENGE_DIR="challenges/$CHALLENGE_NAME"

# Create a new Cargo project for the challenge
cargo new --lib "$CHALLENGE_DIR"

# Create the main.rs file in the challenge directory
cat <<EOT > "$CHALLENGE_DIR/src/main.rs"
use rust_coding_challenges::utils::read_text_file_from_args;

fn main() -> std::io::Result<()> {
    let content = read_text_file_from_args()?;
    let result = solve(&content);
    
    // Print results or perform other actions as needed
    Ok(())
}
EOT

# Create the test file for the challenge
mkdir -p "$CHALLENGE_DIR/tests/inputs"

cat <<EOT > "$CHALLENGE_DIR/tests/test.rs"
#[cfg(test)]
mod tests {

    #[test]
    fn test() {
    }
}
EOT

# Create the README.md file
README_PATH="$CHALLENGE_DIR/README.md"
echo "# $CHALLENGE_NAME" > "$README_PATH"
echo "" >> "$README_PATH"
echo "## Given By $AUTHOR_NAME" >> "$README_PATH"
echo "" >> "$README_PATH"
echo "## Relevant Background Knowledge" >> "$README_PATH"
echo "" >> "$README_PATH"
echo "## Main Functions" >> "$README_PATH"
echo "" >> "$README_PATH"
echo "## Example Output" >> "$README_PATH"
echo "" >> "$README_PATH"
echo "## Usage Example" >> "$README_PATH"
echo "" >> "$README_PATH"

echo "Challenge '$CHALLENGE_NAME' created successfully!"