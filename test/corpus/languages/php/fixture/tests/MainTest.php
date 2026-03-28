<?php

namespace LetaFixture\Tests;

use LetaFixture\Main;
use LetaFixture\MemoryStorage;
use LetaFixture\UserRepository;

class MainTest {
    public function testCreateSampleUser(): void {
        $user = Main::createSampleUser();
    }

    public function testUserRepository(): void {
        $storage = new MemoryStorage();
        $repo = new UserRepository($storage);
        $user = Main::createSampleUser();
        $repo->addUser($user);
    }
}
