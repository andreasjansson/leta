import React from 'react';

/**
 * Error for invalid user input.
 */
export class ValidationError extends Error {
    constructor(public field: string, message: string) {
        super(message);
        this.name = 'ValidationError';
    }
}

/**
 * Error for network failures.
 */
export class NetworkError extends Error {
    constructor(public statusCode: number, message: string) {
        super(message);
        this.name = 'NetworkError';
    }
}

/**
 * Validates required fields.
 */
export function validateRequired(value: string, fieldName: string): void {
    if (!value || value.trim() === '') {
        throw new ValidationError(fieldName, `${fieldName} is required`);
    }
}

/**
 * Error message component.
 */
export function ErrorMessage({ error }: { error: Error }) {
    return (
        <div className="error-message" role="alert">
            <strong>{error.name}:</strong> {error.message}
        </div>
    );
}
