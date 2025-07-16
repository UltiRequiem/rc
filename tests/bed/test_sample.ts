// TypeScript interface
interface User {
    name: string; // user's name
    age: number; /* user's age */
}

/* 
 * This is a TypeScript class
 * with comments
 */
class UserService {
    private users: User[] = [];
    
    // Add a new user
    addUser(user: User): void {
        this.users.push(user);
    }
    
    /* Get user by name */
    getUser(name: string): User | undefined {
        return this.users.find(u => u.name === name);
    }
}

// Export the service
export { UserService };
