from dataclasses import dataclass, field
from typing import Any


@dataclass
class RuntimeState:
    """Mutable runtime state shared across the application."""

    redis_connected: bool = False
    event_subscriber_running: bool = False
    metadata: dict[str, Any] = field(default_factory=dict)
