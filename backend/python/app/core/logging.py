import logging
from logging.config import dictConfig
from typing import Union

LogLevel = Union[int, str]


def configure_logging(level: LogLevel = "INFO") -> None:
    """Configure structured logging for the service."""

    dictConfig(
        {
            "version": 1,
            "disable_existing_loggers": False,
            "formatters": {
                "default": {
                    "format": "%(asctime)s - %(name)s - %(levelname)s - %(message)s",
                },
            },
            "handlers": {
                "console": {
                    "class": "logging.StreamHandler",
                    "formatter": "default",
                    "level": level,
                }
            },
            "root": {
                "handlers": ["console"],
                "level": level,
            },
        }
    )


logger = logging.getLogger("olympus.analytics")
