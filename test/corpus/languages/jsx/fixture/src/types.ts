/**
 * Core types for the application.
 */

/**
 * Represents a user in the system.
 */
export interface User {
    id: string;
    name: string;
    email: string;
    role: UserRole;
}

/**
 * Available user roles.
 */
export type UserRole = 'admin' | 'editor' | 'viewer';

/**
 * Props for components that display a user.
 */
export interface UserDisplayProps {
    user: User;
    onSelect?: (user: User) => void;
}

/**
 * Props for list components.
 */
export interface ListProps<T> {
    items: T[];
    renderItem: (item: T) => React.ReactNode;
    emptyMessage?: string;
}

/**
 * Form field configuration.
 */
export interface FormField {
    name: string;
    label: string;
    type: 'text' | 'email' | 'password' | 'select';
    required?: boolean;
    options?: string[];
}

/**
 * Theme configuration.
 */
export interface ThemeConfig {
    primaryColor: string;
    secondaryColor: string;
    fontFamily: string;
}

/**
 * Default theme values.
 */
export const DEFAULT_THEME: ThemeConfig = {
    primaryColor: '#007bff',
    secondaryColor: '#6c757d',
    fontFamily: 'Arial, sans-serif',
};

/**
 * Status indicators for async operations.
 */
export const STATUS_LABELS: Record<string, string> = {
    loading: 'Loading...',
    success: 'Success',
    error: 'Error occurred',
    idle: 'Ready',
};
