from typing import Any


def get_client_capabilities() -> dict[str, Any]:
    return {
        "workspace": {
            "applyEdit": True,
            "workspaceEdit": {
                "documentChanges": True,
                "resourceOperations": ["create", "rename", "delete"],
            },
            "symbol": {
                "dynamicRegistration": False,
                "symbolKind": {"valueSet": list(range(1, 27))},
            },
            "executeCommand": {"dynamicRegistration": False},
            "workspaceFolders": True,
        },
    }
