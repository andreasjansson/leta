from typing import Any


def get_client_capabilities() -> dict[str, Any]:
    return {
        "workspace": {
            "applyEdit": True,
            "workspaceEdit": {
                "documentChanges": True,
                "resourceOperations": ["create", "rename", "delete"],
            },
            "didChangeConfiguration": {"dynamicRegistration": False},
            "didChangeWatchedFiles": {"dynamicRegistration": False},
            "symbol": {
                "dynamicRegistration": False,
                "symbolKind": {"valueSet": list(range(1, 27))},
            },
            "executeCommand": {"dynamicRegistration": False},
            "workspaceFolders": True,
            "configuration": True,
        },
        "textDocument": {
            "synchronization": {
                "dynamicRegistration": False,
                "willSave": True,
                "willSaveWaitUntil": True,
                "didSave": True,
            },
            "completion": {
                "dynamicRegistration": False,
                "completionItem": {
                    "snippetSupport": False,
                    "documentationFormat": ["plaintext", "markdown"],
                    "resolveSupport": {"properties": ["documentation", "detail"]},
                },
                "completionItemKind": {"valueSet": list(range(1, 26))},
            },
            "hover": {
                "dynamicRegistration": False,
                "contentFormat": ["markdown", "plaintext"],
            },
            "signatureHelp": {
                "dynamicRegistration": False,
                "signatureInformation": {
                    "documentationFormat": ["markdown", "plaintext"],
                    "parameterInformation": {"labelOffsetSupport": True},
                },
            },
            "declaration": {"dynamicRegistration": False, "linkSupport": True},
            "definition": {"dynamicRegistration": False, "linkSupport": True},
            "typeDefinition": {"dynamicRegistration": False, "linkSupport": True},
            "implementation": {"dynamicRegistration": False, "linkSupport": True},
            "references": {"dynamicRegistration": False},
            "documentHighlight": {"dynamicRegistration": False},
            "documentSymbol": {
                "dynamicRegistration": False,
                "symbolKind": {"valueSet": list(range(1, 27))},
                "hierarchicalDocumentSymbolSupport": True,
            },
            "codeAction": {
                "dynamicRegistration": False,
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
                            "source.fixAll",
                        ]
                    }
                },
                "isPreferredSupport": True,
                "resolveSupport": {"properties": ["edit"]},
            },
            "codeLens": {"dynamicRegistration": False},
            "documentLink": {"dynamicRegistration": False},
            "formatting": {"dynamicRegistration": False},
            "rangeFormatting": {"dynamicRegistration": False},
            "onTypeFormatting": {"dynamicRegistration": False},
            "rename": {"dynamicRegistration": False, "prepareSupport": True},
            "publishDiagnostics": {
                "relatedInformation": True,
                "versionSupport": True,
                "codeDescriptionSupport": True,
                "dataSupport": True,
            },
            "callHierarchy": {"dynamicRegistration": False},
            "typeHierarchy": {"dynamicRegistration": False},
            "inlayHint": {"dynamicRegistration": False},
        },
        "window": {
            "showMessage": {"messageActionItem": {"additionalPropertiesSupport": False}},
            "showDocument": {"support": True},
            "workDoneProgress": True,
        },
        "general": {
            "positionEncodings": ["utf-16"],
        },
    }
