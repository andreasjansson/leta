from app import create_service


def test_create_service():
    svc = create_service("test")
    assert svc is not None
