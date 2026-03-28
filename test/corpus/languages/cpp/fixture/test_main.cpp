#include "user.hpp"

void test_create_sample_user() {
    auto user = example::createSampleUser();
}

void test_user_repository() {
    auto storage = std::make_unique<example::MemoryStorage>();
    example::UserRepository repo(std::move(storage));
    auto user = example::createSampleUser();
    repo.addUser(user);
}
