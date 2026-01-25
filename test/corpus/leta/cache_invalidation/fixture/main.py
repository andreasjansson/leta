from user import User


def create_user() -> User:
    return User("test", "test@example.com")


def main():
    user = create_user()
    print(user.name)
