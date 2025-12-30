package com.example;

public class FileStorage extends AbstractStorage {
    private String basePath;

    public FileStorage(String basePath) {
        super("file");
        this.basePath = basePath;
    }

    @Override
    public void save(User user) {
        // Write to file (stub)
    }

    @Override
    public User load(String email) {
        // Read from file (stub)
        return null;
    }
}
