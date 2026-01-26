import React, { useState, useCallback } from 'react';
import { User, DEFAULT_THEME } from './types';
import { useUser, useToggle } from './hooks';
import {
    Button,
    Card,
    UserList,
    Modal,
    Expandable,
    Spinner,
    ThemeProvider,
} from './components';

/**
 * Sample user data for demonstration.
 */
const SAMPLE_USERS: User[] = [
    { id: '1', name: 'Alice Johnson', email: 'alice@example.com', role: 'admin' },
    { id: '2', name: 'Bob Smith', email: 'bob@example.com', role: 'editor' },
    { id: '3', name: 'Carol White', email: 'carol@example.com', role: 'viewer' },
];

/**
 * Header component with navigation.
 */
function Header({ title, onMenuClick }: { title: string; onMenuClick?: () => void }) {
    return (
        <header className="app-header">
            {onMenuClick && (
                <button className="menu-btn" onClick={onMenuClick}>
                    ☰
                </button>
            )}
            <h1>{title}</h1>
        </header>
    );
}

/**
 * Sidebar navigation component.
 */
function Sidebar({ isOpen, onClose }: { isOpen: boolean; onClose: () => void }) {
    if (!isOpen) {
        return null;
    }

    return (
        <nav className="sidebar">
            <button className="close-btn" onClick={onClose}>×</button>
            <ul className="nav-links">
                <li><a href="#dashboard">Dashboard</a></li>
                <li><a href="#users">Users</a></li>
                <li><a href="#settings">Settings</a></li>
            </ul>
        </nav>
    );
}

/**
 * User details panel component.
 */
function UserDetailsPanel({
    user,
    onClose,
}: {
    user: User | null;
    onClose: () => void;
}) {
    if (!user) {
        return null;
    }

    return (
        <Modal isOpen={true} onClose={onClose} title="User Details">
            <div className="user-details">
                <p><strong>Name:</strong> {user.name}</p>
                <p><strong>Email:</strong> {user.email}</p>
                <p><strong>Role:</strong> {user.role}</p>
                <p><strong>ID:</strong> {user.id}</p>
            </div>
        </Modal>
    );
}

/**
 * Dashboard statistics component.
 */
function DashboardStats({ users }: { users: User[] }) {
    const adminCount = users.filter(u => u.role === 'admin').length;
    const editorCount = users.filter(u => u.role === 'editor').length;
    const viewerCount = users.filter(u => u.role === 'viewer').length;

    return (
        <Card title="User Statistics">
            <div className="stats">
                <div className="stat">
                    <span className="stat-value">{users.length}</span>
                    <span className="stat-label">Total Users</span>
                </div>
                <div className="stat">
                    <span className="stat-value">{adminCount}</span>
                    <span className="stat-label">Admins</span>
                </div>
                <div className="stat">
                    <span className="stat-value">{editorCount}</span>
                    <span className="stat-label">Editors</span>
                </div>
                <div className="stat">
                    <span className="stat-value">{viewerCount}</span>
                    <span className="stat-label">Viewers</span>
                </div>
            </div>
        </Card>
    );
}

/**
 * Main application component.
 */
export function App() {
    const [users] = useState<User[]>(SAMPLE_USERS);
    const [selectedUser, setSelectedUser] = useState<User | null>(null);
    const { value: isSidebarOpen, toggle: toggleSidebar, setFalse: closeSidebar } = useToggle(false);
    const [isLoading, setIsLoading] = useState(false);

    const handleUserSelect = useCallback((user: User) => {
        setSelectedUser(user);
    }, []);

    const handleCloseUserDetails = useCallback(() => {
        setSelectedUser(null);
    }, []);

    if (isLoading) {
        return (
            <div className="loading-container">
                <Spinner size="large" />
            </div>
        );
    }

    return (
        <ThemeProvider theme={DEFAULT_THEME}>
            <div className="app">
                <Header title="User Management" onMenuClick={toggleSidebar} />
                <Sidebar isOpen={isSidebarOpen} onClose={closeSidebar} />
                
                <main className="app-content">
                    <DashboardStats users={users} />
                    
                    <Expandable title="User List" defaultExpanded={true}>
                        <UserList users={users} onUserSelect={handleUserSelect} />
                    </Expandable>
                    
                    <Card title="Actions">
                        <div className="action-buttons">
                            <Button variant="primary">Add User</Button>
                            <Button variant="secondary">Import</Button>
                            <Button variant="danger">Clear All</Button>
                        </div>
                    </Card>
                </main>

                <UserDetailsPanel user={selectedUser} onClose={handleCloseUserDetails} />
            </div>
        </ThemeProvider>
    );
}

export default App;
