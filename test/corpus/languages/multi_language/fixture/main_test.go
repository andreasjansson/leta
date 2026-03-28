package main

import "testing"

func TestCreateService(t *testing.T) {
	svc := CreateService("test")
	if svc.Name != "test" {
		t.Errorf("expected test, got %s", svc.Name)
	}
}
