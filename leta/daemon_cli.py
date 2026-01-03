"""Entry point for the leta daemon."""

import asyncio


def main():
    """Run the leta daemon with Unix socket server."""
    from .daemon.server import run_daemon

    asyncio.run(run_daemon())


if __name__ == "__main__":
    main()
