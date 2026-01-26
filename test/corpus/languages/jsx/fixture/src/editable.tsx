/**
 * ISOLATED TEST FILE - Used exclusively by rename and mv tests.
 * Do NOT import this from App.tsx or other non-editable files.
 * Do NOT use symbols from this file in grep, refs, calls, or other read-only tests.
 */

import React from 'react';

/**
 * Editable button component for testing rename operations.
 */
export function EditableButton({
    label,
    onClick,
}: {
    label: string;
    onClick?: () => void;
}) {
    return (
        <button className="editable-btn" onClick={onClick}>
            {label}
        </button>
    );
}

/**
 * Editable container component.
 */
export function EditableContainer({ children }: { children: React.ReactNode }) {
    return <div className="editable-container">{children}</div>;
}

/**
 * Creates editable sample elements.
 */
export function editableCreateElements(): React.ReactNode {
    return (
        <EditableContainer>
            <EditableButton label="Test" />
        </EditableContainer>
    );
}
