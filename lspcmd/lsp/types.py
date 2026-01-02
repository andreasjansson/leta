from enum import IntEnum
from typing import Any, Literal
from pydantic import BaseModel, Field, ConfigDict


class LSPModel(BaseModel):
    """Base model with snake_case alias support for Pythonic usage."""
    model_config = ConfigDict(populate_by_name=True)


class Position(LSPModel):
    line: int
    character: int


class Range(LSPModel):
    start: Position
    end: Position


class Location(LSPModel):
    uri: str
    range: Range


class LocationLink(LSPModel):
    originSelectionRange: Range | None = Field(default=None, alias="origin_selection_range")
    targetUri: str = Field(alias="target_uri")
    targetRange: Range = Field(alias="target_range")
    targetSelectionRange: Range = Field(alias="target_selection_range")


class TextDocumentIdentifier(LSPModel):
    uri: str


class VersionedTextDocumentIdentifier(TextDocumentIdentifier):
    version: int


class OptionalVersionedTextDocumentIdentifier(TextDocumentIdentifier):
    version: int | None = None


class TextDocumentItem(LSPModel):
    uri: str
    languageId: str = Field(alias="language_id")
    version: int
    text: str


class TextDocumentPositionParams(LSPModel):
    textDocument: TextDocumentIdentifier = Field(alias="text_document")
    position: Position


class TextEdit(LSPModel):
    range: Range
    newText: str = Field(alias="new_text")


class AnnotatedTextEdit(TextEdit):
    annotationId: str | None = Field(default=None, alias="annotation_id")


class TextDocumentEdit(LSPModel):
    textDocument: OptionalVersionedTextDocumentIdentifier = Field(alias="text_document")
    edits: list[TextEdit | AnnotatedTextEdit]


class CreateFileOptions(LSPModel):
    overwrite: bool | None = None
    ignoreIfExists: bool | None = Field(default=None, alias="ignore_if_exists")


class CreateFile(LSPModel):
    kind: Literal["create"] = "create"
    uri: str
    options: CreateFileOptions | None = None


class RenameFileOptions(LSPModel):
    overwrite: bool | None = None
    ignoreIfExists: bool | None = Field(default=None, alias="ignore_if_exists")


class RenameFile(LSPModel):
    kind: Literal["rename"] = "rename"
    oldUri: str = Field(alias="old_uri")
    newUri: str = Field(alias="new_uri")
    options: RenameFileOptions | None = None


class DeleteFileOptions(LSPModel):
    recursive: bool | None = None
    ignoreIfNotExists: bool | None = Field(default=None, alias="ignore_if_not_exists")


class DeleteFile(LSPModel):
    kind: Literal["delete"] = "delete"
    uri: str
    options: DeleteFileOptions | None = None


class WorkspaceEdit(LSPModel):
    changes: dict[str, list[TextEdit]] | None = None
    documentChanges: list[TextDocumentEdit | CreateFile | RenameFile | DeleteFile] | None = Field(
        default=None, alias="document_changes"
    )


class Command(LSPModel):
    title: str
    command: str
    arguments: list[Any] | None = None


class SymbolKind(IntEnum):
    File = 1
    Module = 2
    Namespace = 3
    Package = 4
    Class = 5
    Method = 6
    Property = 7
    Field = 8
    Constructor = 9
    Enum = 10
    Interface = 11
    Function = 12
    Variable = 13
    Constant = 14
    String = 15
    Number = 16
    Boolean = 17
    Array = 18
    Object = 19
    Key = 20
    Null = 21
    EnumMember = 22
    Struct = 23
    Event = 24
    Operator = 25
    TypeParameter = 26


class SymbolInformation(LSPModel):
    name: str
    kind: int
    location: Location
    containerName: str | None = Field(default=None, alias="container_name")


class DocumentSymbol(LSPModel):
    name: str
    kind: int
    range: Range
    selectionRange: Range = Field(alias="selection_range")
    detail: str | None = None
    children: list["DocumentSymbol"] | None = None


class Diagnostic(LSPModel):
    range: Range
    message: str
    severity: int | None = None
    code: str | int | None = None
    source: str | None = None


class CodeActionKind:
    Empty = ""
    QuickFix = "quickfix"
    Refactor = "refactor"
    RefactorExtract = "refactor.extract"
    RefactorInline = "refactor.inline"
    RefactorRewrite = "refactor.rewrite"
    Source = "source"
    SourceOrganizeImports = "source.organizeImports"
    SourceFixAll = "source.fixAll"


class CodeAction(LSPModel):
    title: str
    kind: str | None = None
    diagnostics: list[Diagnostic] | None = None
    isPreferred: bool | None = Field(default=None, alias="is_preferred")
    edit: WorkspaceEdit | None = None
    command: Command | None = None
    data: Any | None = None


class MarkupKind:
    PlainText = "plaintext"
    Markdown = "markdown"


class MarkupContent(LSPModel):
    kind: str
    value: str


class Hover(LSPModel):
    contents: MarkupContent | str | list[str]
    range: Range | None = None


class CompletionItemKind(IntEnum):
    Text = 1
    Method = 2
    Function = 3
    Constructor = 4
    Field = 5
    Variable = 6
    Class = 7
    Interface = 8
    Module = 9
    Property = 10
    Unit = 11
    Value = 12
    Enum = 13
    Keyword = 14
    Snippet = 15
    Color = 16
    File = 17
    Reference = 18
    Folder = 19
    EnumMember = 20
    Constant = 21
    Struct = 22
    Event = 23
    Operator = 24
    TypeParameter = 25


class CompletionItem(LSPModel):
    label: str
    kind: int | None = None
    detail: str | None = None
    documentation: MarkupContent | str | None = None
    insertText: str | None = Field(default=None, alias="insert_text")
    textEdit: TextEdit | None = Field(default=None, alias="text_edit")


class CompletionList(LSPModel):
    isIncomplete: bool = Field(alias="is_incomplete")
    items: list[CompletionItem]


class SignatureInformation(LSPModel):
    label: str
    documentation: MarkupContent | str | None = None
    parameters: list["ParameterInformation"] | None = None


class ParameterInformation(LSPModel):
    label: str | tuple[int, int]
    documentation: MarkupContent | str | None = None


class SignatureHelp(LSPModel):
    signatures: list[SignatureInformation]
    activeSignature: int | None = Field(default=None, alias="active_signature")
    activeParameter: int | None = Field(default=None, alias="active_parameter")


class FormattingOptions(LSPModel):
    tabSize: int = Field(alias="tab_size")
    insertSpaces: bool = Field(alias="insert_spaces")
    trimTrailingWhitespace: bool | None = Field(default=None, alias="trim_trailing_whitespace")
    insertFinalNewline: bool | None = Field(default=None, alias="insert_final_newline")
    trimFinalNewlines: bool | None = Field(default=None, alias="trim_final_newlines")


class ReferenceContext(LSPModel):
    includeDeclaration: bool = Field(alias="include_declaration")


class CallHierarchyItem(LSPModel):
    name: str
    kind: int
    uri: str
    range: Range
    selectionRange: Range = Field(alias="selection_range")
    detail: str | None = None
    data: Any | None = None


class CallHierarchyIncomingCall(LSPModel):
    from_: CallHierarchyItem = Field(alias="from")
    fromRanges: list[Range] = Field(alias="from_ranges")


class CallHierarchyOutgoingCall(LSPModel):
    to: CallHierarchyItem
    fromRanges: list[Range] = Field(alias="from_ranges")


class TypeHierarchyItem(LSPModel):
    name: str
    kind: int
    uri: str
    range: Range
    selectionRange: Range = Field(alias="selection_range")
    detail: str | None = None
    tags: list[int] | None = None
    data: Any | None = None


class ServerCapabilities(LSPModel, extra="allow"):
    textDocumentSync: int | dict[str, Any] | None = Field(default=None, alias="text_document_sync")
    completionProvider: dict[str, Any] | None = Field(default=None, alias="completion_provider")
    hoverProvider: bool | dict[str, Any] | None = Field(default=None, alias="hover_provider")
    signatureHelpProvider: dict[str, Any] | None = Field(default=None, alias="signature_help_provider")
    declarationProvider: bool | dict[str, Any] | None = Field(default=None, alias="declaration_provider")
    definitionProvider: bool | dict[str, Any] | None = Field(default=None, alias="definition_provider")
    typeDefinitionProvider: bool | dict[str, Any] | None = Field(default=None, alias="type_definition_provider")
    implementationProvider: bool | dict[str, Any] | None = Field(default=None, alias="implementation_provider")
    referencesProvider: bool | dict[str, Any] | None = Field(default=None, alias="references_provider")
    documentHighlightProvider: bool | dict[str, Any] | None = Field(default=None, alias="document_highlight_provider")
    documentSymbolProvider: bool | dict[str, Any] | None = Field(default=None, alias="document_symbol_provider")
    codeActionProvider: bool | dict[str, Any] | None = Field(default=None, alias="code_action_provider")
    codeLensProvider: dict[str, Any] | None = Field(default=None, alias="code_lens_provider")
    documentLinkProvider: dict[str, Any] | None = Field(default=None, alias="document_link_provider")
    colorProvider: bool | dict[str, Any] | None = Field(default=None, alias="color_provider")
    documentFormattingProvider: bool | dict[str, Any] | None = Field(default=None, alias="document_formatting_provider")
    documentRangeFormattingProvider: bool | dict[str, Any] | None = Field(default=None, alias="document_range_formatting_provider")
    documentOnTypeFormattingProvider: dict[str, Any] | None = Field(default=None, alias="document_on_type_formatting_provider")
    renameProvider: bool | dict[str, Any] | None = Field(default=None, alias="rename_provider")
    foldingRangeProvider: bool | dict[str, Any] | None = Field(default=None, alias="folding_range_provider")
    executeCommandProvider: dict[str, Any] | None = Field(default=None, alias="execute_command_provider")
    selectionRangeProvider: bool | dict[str, Any] | None = Field(default=None, alias="selection_range_provider")
    linkedEditingRangeProvider: bool | dict[str, Any] | None = Field(default=None, alias="linked_editing_range_provider")
    callHierarchyProvider: bool | dict[str, Any] | None = Field(default=None, alias="call_hierarchy_provider")
    semanticTokensProvider: dict[str, Any] | None = Field(default=None, alias="semantic_tokens_provider")
    monikerProvider: bool | dict[str, Any] | None = Field(default=None, alias="moniker_provider")
    typeHierarchyProvider: bool | dict[str, Any] | None = Field(default=None, alias="type_hierarchy_provider")
    inlineValueProvider: bool | dict[str, Any] | None = Field(default=None, alias="inline_value_provider")
    inlayHintProvider: bool | dict[str, Any] | None = Field(default=None, alias="inlay_hint_provider")
    diagnosticProvider: dict[str, Any] | None = Field(default=None, alias="diagnostic_provider")
    workspaceSymbolProvider: bool | dict[str, Any] | None = Field(default=None, alias="workspace_symbol_provider")
    workspace: dict[str, Any] | None = None
    experimental: Any | None = None


class ServerInfo(LSPModel):
    name: str
    version: str | None = None


class InitializeResult(LSPModel):
    capabilities: ServerCapabilities
    serverInfo: ServerInfo | None = Field(default=None, alias="server_info")


# =============================================================================
# LSP Request Params
# =============================================================================


class WorkspaceFolder(LSPModel):
    uri: str
    name: str


class ClientCapabilities(LSPModel, extra="allow"):
    workspace: dict[str, Any] | None = None
    textDocument: dict[str, Any] | None = Field(default=None, alias="text_document")
    window: dict[str, Any] | None = None
    general: dict[str, Any] | None = None
    experimental: Any | None = None


class InitializeParams(LSPModel):
    processId: int | None = Field(alias="process_id")
    rootUri: str | None = Field(alias="root_uri")
    rootPath: str | None = Field(default=None, alias="root_path")
    capabilities: ClientCapabilities
    workspaceFolders: list[WorkspaceFolder] | None = Field(default=None, alias="workspace_folders")
    initializationOptions: Any | None = Field(default=None, alias="initialization_options")
    trace: str | None = None


class ReferenceParams(TextDocumentPositionParams):
    context: ReferenceContext


class DocumentSymbolParams(LSPModel):
    textDocument: TextDocumentIdentifier = Field(alias="text_document")


class RenameParams(LSPModel):
    textDocument: TextDocumentIdentifier = Field(alias="text_document")
    position: Position
    newName: str = Field(alias="new_name")


class CallHierarchyItemParams(LSPModel):
    item: CallHierarchyItem


class TypeHierarchyItemParams(LSPModel):
    item: TypeHierarchyItem


class FileRename(LSPModel):
    oldUri: str = Field(alias="old_uri")
    newUri: str = Field(alias="new_uri")


class RenameFilesParams(LSPModel):
    files: list[FileRename]


# =============================================================================
# LSP Response Type Aliases
# =============================================================================

DefinitionResponse = Location | list[Location] | list[LocationLink] | None
DeclarationResponse = Location | list[Location] | list[LocationLink] | None
ReferencesResponse = list[Location] | None
ImplementationResponse = Location | list[Location] | list[LocationLink] | None
TypeDefinitionResponse = Location | list[Location] | list[LocationLink] | None
HoverResponse = Hover | None
DocumentSymbolResponse = list[DocumentSymbol] | list[SymbolInformation] | None
RenameResponseType = WorkspaceEdit | None
PrepareCallHierarchyResponse = list[CallHierarchyItem] | None
CallHierarchyIncomingCallsResponse = list[CallHierarchyIncomingCall] | None
CallHierarchyOutgoingCallsResponse = list[CallHierarchyOutgoingCall] | None
PrepareTypeHierarchyResponse = list[TypeHierarchyItem] | None
TypeHierarchySubtypesResponse = list[TypeHierarchyItem] | None
TypeHierarchySupertypesResponse = list[TypeHierarchyItem] | None
WillRenameFilesResponse = WorkspaceEdit | None
