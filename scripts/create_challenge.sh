#!/bin/bash

# Create the challenge folder structure
CHALLENGE_NAME=$1
if [ -z "$CHALLENGE_NAME" ]; then
    echo "Please provide the challenge name."
    exit 1
fi

# Create a new Cargo project for the challenge
cargo new "challenges/$CHALLENGE_NAME"

# Create the tests folder
mkdir "challenges/$CHALLENGE_NAME/tests"

# Create a test file in the tests folder
touch "challenges/$CHALLENGE_NAME/tests/test_${CHALLENGE_NAME}.rs"

echo "Challenge structure created for $CHALLENGE_NAME."