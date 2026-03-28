package com.example;

public class MainTest {
    public void testCreateSampleUser() {
        User user = Main.createSampleUser();
        assert user.getName().equals("John");
    }

    public void testUserRepository() {
        Storage storage = new MemoryStorage();
        UserRepository repo = new UserRepository(storage);
        User user = Main.createSampleUser();
        repo.addUser(user);
    }
}
