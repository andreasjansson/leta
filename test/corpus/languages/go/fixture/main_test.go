package main

import "testing"

func TestCreateSampleUser(t *testing.T) {
	user := createSampleUser()
	if user.Name != "John" {
		t.Errorf("expected John, got %s", user.Name)
	}
}

func TestUserRepository(t *testing.T) {
	storage := NewMemoryStorage()
	repo := NewUserRepository(storage)
	user := createSampleUser()
	err := repo.AddUser(user)
	if err != nil {
		t.Fatal(err)
	}
}
