import { User, UserRepository } from './user';

function createSampleUser(): User {
    return new User("John Doe", "john@example.com", 30);
}

function main(): void {
    const repo = new UserRepository();
    const user = createSampleUser();
    repo.addUser(user);

    const found = repo.getUser("john@example.com");
    if (found) {
        console.log(`Found user: ${found.name}`);
    }
}

main();
