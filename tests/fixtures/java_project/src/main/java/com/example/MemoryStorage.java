package com.example;

import java.util.HashMap;
import java.util.Map;

public class MemoryStorage extends AbstractStorage {
    private Map<String, User> users = new HashMap<>();

    public MemoryStorage() {
        super("memory");
    }

    @Override
    public void save(User user) {
        users.put(user.getEmail(), user);
    }

    @Override
    public User load(String email) {
        return users.get(email);
    }
}
