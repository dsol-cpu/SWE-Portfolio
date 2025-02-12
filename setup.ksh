#!/bin/ksh

VENV_DIR="portfolio-venv"
ACTIVATE_SCRIPT="$VENV_DIR/bin/activate"

# Check if the virtual environment is already set up
if [ -f "$ACTIVATE_SCRIPT" ]; then
    . "$ACTIVATE_SCRIPT" >/dev/null 2>&1
    if [ -n "$VIRTUAL_ENV" ]; then
        echo "Virtual environment '$VENV_DIR' is already active."
    else
        echo "Activating virtual environment..."
        . "$ACTIVATE_SCRIPT"
    fi
else
    echo "Creating Python virtual environment: $VENV_DIR"
    python3 -m venv "$VENV_DIR"

    if [ -f "$ACTIVATE_SCRIPT" ]; then
        echo "Virtual environment '$VENV_DIR' created successfully."
        . "$ACTIVATE_SCRIPT"
    else
        echo "Failed to create virtual environment."
        exit 1
    fi
fi

# Check if ggshield is already installed
if ! pip show ggshield >/dev/null 2>&1; then
    echo "Installing ggshield..."
    pip install ggshield
else
    echo "ggshield is already installed."
fi
