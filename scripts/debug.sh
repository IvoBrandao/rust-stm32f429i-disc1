#!/bin/sh
# Runner script for `cargo run` / `make debug`.
# Starts OpenOCD in the background, launches GDB with the ELF passed by cargo,
# then kills OpenOCD when GDB exits.
openocd -f openocd.cfg &
OPENOCD_PID=$!
sleep 0.5
gdb -q -x openocd.gdb "$@"
EXIT_CODE=$?
kill $OPENOCD_PID 2>/dev/null
wait $OPENOCD_PID 2>/dev/null
exit $EXIT_CODE
