from main import create_sample_user, UserRepository


def test_create_sample_user():
    user = create_sample_user()
    assert user.name == "John"


def test_user_repository():
    repo = UserRepository()
    user = create_sample_user()
    repo.add_user(user)
    assert repo.count_users() == 1
