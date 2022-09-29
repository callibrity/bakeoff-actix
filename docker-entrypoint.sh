#!/bin/bash

# Apply database migrations
echo "Applying database migrations..."
diesel migration run

# Start server
echo "Starting server"
cargo run --release --bin bakeoff-rust