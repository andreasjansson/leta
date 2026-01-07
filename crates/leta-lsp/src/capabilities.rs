use serde_json::{json, Value};

pub fn get_client_capabilities() -> Value {
    json!({
        "experimental": {
            "serverStatusNotification": true
        },
        "workspace": {
            "applyEdit": true,
            "workspaceEdit": {
                "documentChanges": true,
                "resourceOperations": ["create", "rename", "delete"]
            },
            "symbol": {
                "dynamicRegistration": false,
                "symbolKind": {
                    "valueSet": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26]
                }
            },
            "executeCommand": {
                "dynamicRegistration": false
            },
            "fileOperations": {
                "dynamicRegistration": false,
                "willRename": true,
                "didRename": true
            }
        },
        "textDocument": {
            "synchronization": {
                "dynamicRegistration": false,
                "didSave": true
            },
            "hover": {
                "dynamicRegistration": false,
                "contentFormat": ["markdown", "plaintext"]
            },
            "declaration": {
                "dynamicRegistration": false,
                "linkSupport": true
            },
            "definition": {
                "dynamicRegistration": false,
                "linkSupport": true
            },
            "typeDefinition": {
                "dynamicRegistration": false,
                "linkSupport": true
            },
            "implementation": {
                "dynamicRegistration": false,
                "linkSupport": true
            },
            "references": {
                "dynamicRegistration": false
            },
            "documentSymbol": {
                "dynamicRegistration": false,
                "symbolKind": {
                    "valueSet": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26]
                },
                "hierarchicalDocumentSymbolSupport": true
            },
            "codeAction": {
                "dynamicRegistration": false,
                "codeActionLiteralSupport": {
                    "codeActionKind": {
                        "valueSet": [
                            "",
                            "quickfix",
                            "refactor",
                            "refactor.extract",
                            "refactor.inline",
                            "refactor.rewrite",
                            "source",
                            "source.organizeImports",
                            "source.fixAll"
                        ]
                    }
                },
                "isPreferredSupport": true,
                "resolveSupport": {
                    "properties": ["edit"]
                }
            },
            "formatting": {
                "dynamicRegistration": false
            },
            "rangeFormatting": {
                "dynamicRegistration": false
            },
            "rename": {
                "dynamicRegistration": false,
                "prepareSupport": true
            },
            "publishDiagnostics": {
                "relatedInformation": true
            },
            "callHierarchy": {
                "dynamicRegistration": false
            },
            "typeHierarchy": {
                "dynamicRegistration": false
            }
        },
        "window": {
            "workDoneProgress": true
        }
    })
}
