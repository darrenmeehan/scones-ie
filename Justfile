ship:
    echo "Let's ship it!"
    cargo fmt
    cargo test
    fly deploy
