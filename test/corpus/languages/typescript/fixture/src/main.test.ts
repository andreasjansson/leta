import { createSampleUser } from "./main";
import { UserRepository, MemoryStorage } from "./user";

function testCreateSampleUser() {
    const user = createSampleUser();
    console.assert(user.name === "John");
}

function testUserRepository() {
    const storage = new MemoryStorage();
    const repo = new UserRepository(storage);
    const user = createSampleUser();
    repo.addUser(user);
    console.assert(repo.countUsers() === 1);
}
