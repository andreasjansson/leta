use crate::types::*;

pub fn get_client_capabilities() -> ClientCapabilities {
    ClientCapabilities {
        workspace: Some(WorkspaceClientCapabilities {
            workspace_edit: Some(WorkspaceEditCapabilities {
                document_changes: Some(true),
            }),
            file_operations: Some(FileOperationsCapabilities {
                will_rename: Some(true),
            }),
        }),
        text_document: Some(TextDocumentClientCapabilities {
            definition: Some(DefinitionCapabilities {
                link_support: Some(true),
            }),
            references: Some(ReferencesCapabilities {}),
            document_symbol: Some(DocumentSymbolCapabilities {
                hierarchical_document_symbol_support: Some(true),
            }),
            rename: Some(RenameCapabilities {
                prepare_support: Some(true),
            }),
            hover: Some(HoverCapabilities {
                content_format: Some(vec!["markdown".to_string(), "plaintext".to_string()]),
            }),
            call_hierarchy: Some(CallHierarchyCapabilities {}),
            type_hierarchy: Some(TypeHierarchyCapabilities {}),
        }),
    }
}
