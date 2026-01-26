import React, { useState, useCallback } from 'react';
import { User, UserDisplayProps, ListProps, ThemeConfig } from './types';
import { useToggle } from './hooks';

/**
 * Button component with various styles.
 */
export function Button({
    children,
    onClick,
    variant = 'primary',
    disabled = false,
}: {
    children: React.ReactNode;
    onClick?: () => void;
    variant?: 'primary' | 'secondary' | 'danger';
    disabled?: boolean;
}) {
    return (
        <button
            className={`btn btn-${variant}`}
            onClick={onClick}
            disabled={disabled}
        >
            {children}
        </button>
    );
}

/**
 * Card component for content containers.
 */
export function Card({
    title,
    children,
    footer,
}: {
    title?: string;
    children: React.ReactNode;
    footer?: React.ReactNode;
}) {
    return (
        <div className="card">
            {title && <div className="card-header">{title}</div>}
            <div className="card-body">{children}</div>
            {footer && <div className="card-footer">{footer}</div>}
        </div>
    );
}

/**
 * User avatar component.
 */
export function UserAvatar({ user, size = 40 }: { user: User; size?: number }) {
    const initials = user.name
        .split(' ')
        .map(n => n[0])
        .join('')
        .toUpperCase();

    return (
        <div
            className="avatar"
            style={{
                width: size,
                height: size,
                borderRadius: '50%',
                backgroundColor: '#007bff',
                color: 'white',
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
                fontSize: size / 2,
            }}
        >
            {initials}
        </div>
    );
}

/**
 * User card component showing user details.
 */
export function UserCard({ user, onSelect }: UserDisplayProps) {
    const handleClick = useCallback(() => {
        if (onSelect) {
            onSelect(user);
        }
    }, [user, onSelect]);

    return (
        <Card title={user.name}>
            <div className="user-card">
                <UserAvatar user={user} />
                <div className="user-info">
                    <p className="email">{user.email}</p>
                    <span className={`role role-${user.role}`}>{user.role}</span>
                </div>
            </div>
            {onSelect && (
                <Button onClick={handleClick} variant="secondary">
                    Select
                </Button>
            )}
        </Card>
    );
}

/**
 * Generic list component.
 */
export function List<T>({ items, renderItem, emptyMessage = 'No items' }: ListProps<T>) {
    if (items.length === 0) {
        return <div className="empty-list">{emptyMessage}</div>;
    }

    return (
        <ul className="list">
            {items.map((item, index) => (
                <li key={index} className="list-item">
                    {renderItem(item)}
                </li>
            ))}
        </ul>
    );
}

/**
 * User list component.
 */
export function UserList({
    users,
    onUserSelect,
}: {
    users: User[];
    onUserSelect?: (user: User) => void;
}) {
    return (
        <List
            items={users}
            renderItem={(user) => (
                <UserCard user={user} onSelect={onUserSelect} />
            )}
            emptyMessage="No users found"
        />
    );
}

/**
 * Modal dialog component.
 */
export function Modal({
    isOpen,
    onClose,
    title,
    children,
}: {
    isOpen: boolean;
    onClose: () => void;
    title: string;
    children: React.ReactNode;
}) {
    if (!isOpen) {
        return null;
    }

    return (
        <div className="modal-overlay" onClick={onClose}>
            <div className="modal-content" onClick={(e) => e.stopPropagation()}>
                <div className="modal-header">
                    <h2>{title}</h2>
                    <button className="close-btn" onClick={onClose}>
                        ×
                    </button>
                </div>
                <div className="modal-body">{children}</div>
            </div>
        </div>
    );
}

/**
 * Expandable section component.
 */
export function Expandable({
    title,
    children,
    defaultExpanded = false,
}: {
    title: string;
    children: React.ReactNode;
    defaultExpanded?: boolean;
}) {
    const { value: isExpanded, toggle } = useToggle(defaultExpanded);

    return (
        <div className="expandable">
            <button className="expandable-header" onClick={toggle}>
                <span className="expandable-icon">{isExpanded ? '▼' : '▶'}</span>
                {title}
            </button>
            {isExpanded && <div className="expandable-content">{children}</div>}
        </div>
    );
}

/**
 * Loading spinner component.
 */
export function Spinner({ size = 'medium' }: { size?: 'small' | 'medium' | 'large' }) {
    const sizes = { small: 16, medium: 32, large: 48 };
    return (
        <div
            className={`spinner spinner-${size}`}
            style={{ width: sizes[size], height: sizes[size] }}
        />
    );
}

/**
 * Error boundary fallback component.
 */
export function ErrorFallback({ error, resetError }: { error: Error; resetError: () => void }) {
    return (
        <Card title="Something went wrong">
            <p className="error-message">{error.message}</p>
            <Button onClick={resetError} variant="primary">
                Try Again
            </Button>
        </Card>
    );
}

/**
 * Theme provider component.
 */
export function ThemeProvider({
    theme,
    children,
}: {
    theme: ThemeConfig;
    children: React.ReactNode;
}) {
    return (
        <div
            className="theme-provider"
            style={
                {
                    '--primary-color': theme.primaryColor,
                    '--secondary-color': theme.secondaryColor,
                    '--font-family': theme.fontFamily,
                } as React.CSSProperties
            }
        >
            {children}
        </div>
    );
}
