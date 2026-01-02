"""
This file contains functions used exclusively by replace-function tests.
Do not use these symbols in other tests to avoid parallel test interference.
"""

from dataclasses import dataclass


@dataclass
class EditableUser:
    name: str
    email: str
    age: int


def editable_create_user() -> EditableUser:
    """Create an editable sample user for testing."""
    return EditableUser(name="Original Name", email="original@example.com", age=30)


def editable_validate_email(email: str) -> bool:
    """Validate an editable email address."""
    return "@" in email


class EditableStorage:
    """Editable storage class for testing method replacement."""

    def __init__(self) -> None:
        self._data: dict[str, str] = {}

    def save(self, key: str, value: str) -> None:
        """Save a value."""
        self._data[key] = value

    def load(self, key: str) -> str | None:
        """Load a value."""
        return self._data.get(key)
