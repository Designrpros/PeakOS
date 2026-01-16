#!/bin/sh
# Boot animation that displays Peak logo with loading animation
# This runs before Weston starts to show something nice during boot

# Clear screen
clear

# Use fbv or fbi to display boot splash if available
if command -v fbv >/dev/null 2>&1; then
    # Display splash image in background
    fbv -c -i /boot_splash.png &
    SPLASH_PID=$!
elif command -v fbi >/dev/null 2>&1; then
    fbi -T 1 -noverbose -a /boot_splash.png &
    SPLASH_PID=$!
else
    # Fallback: ASCII art
    cat << 'EOF'

                    ▲
                   ▲ ▲
                  ▲   ▲
                 ▲▲▲▲▲▲▲

              P e a k O S

            Loading system...

EOF
fi

# Simple loading animation
echo -n "    "
for i in 1 2 3 4 5 6 7 8; do
    echo -n "●"
    sleep 0.3
done
echo ""

# Kill splash if it's still running
if [ -n "$SPLASH_PID" ]; then
    kill $SPLASH_PID 2>/dev/null
fi
