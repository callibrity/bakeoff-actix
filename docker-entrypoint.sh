#!/bin/bash

# Apply database migrations
echo "Applying database migrations..."
diesel migration run

# Start server
echo "Starting server"
/usr/local/bin/bakeoff-rust