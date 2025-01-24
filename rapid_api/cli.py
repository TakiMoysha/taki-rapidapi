import sys

import argparse
import structlog

logger = structlog.get_logger()

def run_cli():
    parser = argparse.ArgumentParser()

    group = parser.add_argument_group("Development", "For package development")
    group.add_argument("--client", action="store_true")

    args = parser.parse_args()
    logger.info("Running", config=args)
