/**
 * ISOLATED TEST FILE - Imports from editable.tsx for testing cross-file rename and mv.
 * Do NOT import this from App.tsx or other non-editable files.
 */

import React from 'react';
import { EditableButton, EditableContainer } from './editable';

/**
 * Uses EditableButton to test that rename propagates across files.
 */
export function useEditableButton(): React.ReactNode {
    return <EditableButton label="Consumer Button" />;
}

/**
 * Uses EditableContainer with content.
 */
export function useEditableContainer(): React.ReactNode {
    return (
        <EditableContainer>
            <p>Content inside container</p>
        </EditableContainer>
    );
}
