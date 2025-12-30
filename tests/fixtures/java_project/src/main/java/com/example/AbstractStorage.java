package com.example;

public abstract class AbstractStorage implements Storage {
    protected String name;

    public AbstractStorage(String name) {
        this.name = name;
    }

    public String getName() {
        return name;
    }

    // Subclasses must implement save and load
}
