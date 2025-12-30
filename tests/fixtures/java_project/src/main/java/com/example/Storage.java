package com.example;

public interface Storage {
    void save(User user);
    User load(String email);
}
