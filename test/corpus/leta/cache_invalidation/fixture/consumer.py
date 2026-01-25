from user import User


def process_user(user: User) -> str:
    return f"{user.name} <{user.email}>"
