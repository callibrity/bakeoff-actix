#!/bin/bash

# Apply database migrations
echo "Applying database migrations..."
python manage.py migrate

# Start server
echo "Starting server"
cargo run --release --bin bakeoff-rust