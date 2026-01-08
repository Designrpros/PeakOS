if [[ -z $DISPLAY ]] && [[ $(tty) = /dev/tty1 ]]; then
  # Start Cage with our App
  # export WEBKIT_DISABLE_COMPOSITING_MODE=1 # Optional fix for some VMs
  exec cage -s -- .cargo-lock
fi
