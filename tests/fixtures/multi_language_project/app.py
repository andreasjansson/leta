"""Python part of the multi-language project."""


class PythonService:
    """A service implemented in Python."""

    def __init__(self, name: str):
        self.name = name

    def greet(self) -> str:
        return f"Hello from Python, {self.name}!"


def create_service(name: str) -> PythonService:
    return PythonService(name)
