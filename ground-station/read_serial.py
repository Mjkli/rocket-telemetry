#!/usr/bin/env python3

import argparse
import time

import serial

TEST_PAYLOAD = bytes(range(1, 11)) + b"\x00"


def main() -> None:
    parser = argparse.ArgumentParser(description="Print bytes received from a serial port.")
    parser.add_argument("--port", default="/dev/serial0")
    parser.add_argument("--baud", type=int, default=9600)
    parser.add_argument(
        "--send-test",
        action="store_true",
        help="send the UART test pattern while listening",
    )
    parser.add_argument("--interval", type=float, default=0.1, help="test send interval in seconds")
    args = parser.parse_args()

    if args.interval <= 0:
        parser.error("--interval must be greater than zero")

    if args.send_test:
        next_send = time.monotonic()

    try:
        with serial.Serial(args.port, args.baud, timeout=0.1, write_timeout=1) as port:
            mode = "sending test pattern and reading" if args.send_test else "reading"
            print(f"{mode.capitalize()} on {args.port} at {args.baud} baud. Press Ctrl+C to stop.", flush=True)
            while True:
                if args.send_test and time.monotonic() >= next_send:
                    port.write(TEST_PAYLOAD)
                    port.flush()
                    print(f"TX {len(TEST_PAYLOAD)} bytes: {TEST_PAYLOAD.hex(' ').upper()}", flush=True)
                    next_send = time.monotonic() + args.interval

                data = port.read(port.in_waiting or 1)
                if data:
                    print(f"RX {len(data)} bytes: {data.hex(' ').upper()} | {data!r}", flush=True)
    except KeyboardInterrupt:
        print("\nStopped.")


if __name__ == "__main__":
    main()