export class User {
    constructor(
        public readonly name: string,
        public readonly email: string,
        public readonly age: number
    ) {}
}

export class UserRepository {
    private users: Map<string, User> = new Map();

    addUser(user: User): void {
        this.users.set(user.email, user);
    }

    getUser(email: string): User | undefined {
        return this.users.get(email);
    }

    deleteUser(email: string): boolean {
        return this.users.delete(email);
    }

    listUsers(): User[] {
        return Array.from(this.users.values());
    }
}
