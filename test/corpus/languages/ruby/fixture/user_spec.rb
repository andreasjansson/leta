require_relative 'user'
require_relative 'main'

def test_create_sample_user
  user = create_sample_user
  raise "wrong name" unless user.name == "John"
end

def test_user_repository
  storage = MemoryStorage.new
  repo = UserRepository.new(storage)
  user = create_sample_user
  repo.add_user(user)
end
