import { useState, useCallback, useEffect } from 'react';
import { User } from './types';

/**
 * Custom hook for managing user state.
 */
export function useUser(initialUser: User | null = null) {
    const [user, setUser] = useState<User | null>(initialUser);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);

    const updateUser = useCallback((updates: Partial<User>) => {
        if (user) {
            setUser({ ...user, ...updates });
        }
    }, [user]);

    const clearUser = useCallback(() => {
        setUser(null);
        setError(null);
    }, []);

    return { user, loading, error, setUser, updateUser, clearUser };
}

/**
 * Custom hook for form handling.
 */
export function useForm<T extends Record<string, string>>(initialValues: T) {
    const [values, setValues] = useState<T>(initialValues);
    const [errors, setErrors] = useState<Partial<Record<keyof T, string>>>({});

    const handleChange = useCallback((name: keyof T, value: string) => {
        setValues(prev => ({ ...prev, [name]: value }));
        setErrors(prev => ({ ...prev, [name]: undefined }));
    }, []);

    const reset = useCallback(() => {
        setValues(initialValues);
        setErrors({});
    }, [initialValues]);

    const validate = useCallback((validators: Partial<Record<keyof T, (value: string) => string | null>>) => {
        const newErrors: Partial<Record<keyof T, string>> = {};
        let isValid = true;

        for (const [key, validator] of Object.entries(validators)) {
            if (validator) {
                const error = validator(values[key as keyof T]);
                if (error) {
                    newErrors[key as keyof T] = error;
                    isValid = false;
                }
            }
        }

        setErrors(newErrors);
        return isValid;
    }, [values]);

    return { values, errors, handleChange, reset, validate };
}

/**
 * Custom hook for toggle state.
 */
export function useToggle(initialValue: boolean = false) {
    const [value, setValue] = useState(initialValue);

    const toggle = useCallback(() => {
        setValue(prev => !prev);
    }, []);

    const setTrue = useCallback(() => setValue(true), []);
    const setFalse = useCallback(() => setValue(false), []);

    return { value, toggle, setTrue, setFalse };
}

/**
 * Custom hook for debounced value.
 */
export function useDebounce<T>(value: T, delay: number): T {
    const [debouncedValue, setDebouncedValue] = useState<T>(value);

    useEffect(() => {
        const handler = setTimeout(() => {
            setDebouncedValue(value);
        }, delay);

        return () => {
            clearTimeout(handler);
        };
    }, [value, delay]);

    return debouncedValue;
}
