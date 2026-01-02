from enum import IntEnum
from typing import Any, Literal
from pydantic import BaseModel, Field, ConfigDict, AliasChoices


class LSPModel(BaseModel):
    """Base model allowing both snake_case and camelCase field access."""
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
    origin_selection_range: Range | None = Field(default=None, alias="originSelectionRange")
    target_uri: str = Field(alias="targetUri")
    target_range: Range = Field(alias="targetRange")
    target_selection_range: Range = Field(alias="targetSelectionRange")


class TextDocumentIdentifier(LSPModel):
    uri: str


class VersionedTextDocumentIdentifier(TextDocumentIdentifier):
    version: int


class OptionalVersionedTextDocumentIdentifier(TextDocumentIdentifier):
    version: int | None = None


class TextDocumentItem(LSPModel):
    uri: str
    language_id: str = Field(alias="languageId")
    version: int
    text: str


class TextDocumentPositionParams(LSPModel):
    text_document: TextDocumentIdentifier = Field(alias="textDocument")
    position: Position


class TextEdit(LSPModel):
    range: Range
    new_text: str = Field(alias="newText")


class AnnotatedTextEdit(TextEdit):
    annotation_id: str | None = Field(default=None, alias="annotationId")


class TextDocumentEdit(LSPModel):
    text_document: OptionalVersionedTextDocumentIdentifier = Field(alias="textDocument")
    edits: list[TextEdit | AnnotatedTextEdit]


class CreateFileOptions(LSPModel):
    overwrite: bool | None = None
    ignore_if_exists: bool | None = Field(default=None, alias="ignoreIfExists")


class CreateFile(LSPModel):
    kind: Literal["create"] = "create"
    uri: str
    options: CreateFileOptions | None = None


class RenameFileOptions(LSPModel):
    overwrite: bool | None = None
    ignore_if_exists: bool | None = Field(default=None, alias="ignoreIfExists")


class RenameFile(LSPModel):
    kind: Literal["rename"] = "rename"
    old_uri: str = Field(alias="oldUri")
    new_uri: str = Field(alias="newUri")
    options: RenameFileOptions | None = None


class DeleteFileOptions(LSPModel):
    recursive: bool | None = None
    ignore_if_not_exists: bool | None = Field(default=None, alias="ignoreIfNotExists")


class DeleteFile(LSPModel):
    kind: Literal["delete"] = "delete"
    uri: str
    options: DeleteFileOptions | None = None


class WorkspaceEdit(LSPModel):
    changes: dict[str, list[TextEdit]] | None = None
    document_changes: list[TextDocumentEdit | CreateFile | RenameFile | DeleteFile] | None = Field(
        default=None, alias="documentChanges"
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
    container_name: str | None = Field(default=None, alias="containerName")


class DocumentSymbol(LSPModel):
    name: str
    kind: int
    range: Range
    selection_range: Range = Field(alias="selectionRange")
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
    is_preferred: bool | None = Field(default=None, alias="isPreferred")
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
    insert_text: str | None = Field(default=None, alias="insertText")
    text_edit: TextEdit | None = Field(default=None, alias="textEdit")


class CompletionList(LSPModel):
    is_incomplete: bool = Field(alias="isIncomplete")
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
    active_signature: int | None = Field(default=None, alias="activeSignature")
    active_parameter: int | None = Field(default=None, alias="activeParameter")


class FormattingOptions(LSPModel):
    tab_size: int = Field(alias="tabSize")
    insert_spaces: bool = Field(alias="insertSpaces")
    trim_trailing_whitespace: bool | None = Field(default=None, alias="trimTrailingWhitespace")
    insert_final_newline: bool | None = Field(default=None, alias="insertFinalNewline")
    trim_final_newlines: bool | None = Field(default=None, alias="trimFinalNewlines")


class ReferenceContext(LSPModel):
    include_declaration: bool = Field(alias="includeDeclaration")


class CallHierarchyItem(LSPModel):
    name: str
    kind: int
    uri: str
    range: Range
    selection_range: Range = Field(alias="selectionRange")
    detail: str | None = None
    data: Any | None = None


class CallHierarchyIncomingCall(LSPModel):
    from_: CallHierarchyItem = Field(alias="from")
    from_ranges: list[Range] = Field(alias="fromRanges")


class CallHierarchyOutgoingCall(LSPModel):
    to: CallHierarchyItem
    from_ranges: list[Range] = Field(alias="fromRanges")


class TypeHierarchyItem(LSPModel):
    name: str
    kind: int
    uri: str
    range: Range
    selection_range: Range = Field(alias="selectionRange")
    detail: str | None = None
    tags: list[int] | None = None
    data: Any | None = None


class ServerCapabilities(LSPModel, extra="allow"):
    pass


class ServerInfo(LSPModel):
    name: str
    version: str | None = None


class InitializeResult(LSPModel):
    capabilities: ServerCapabilities
    server_info: ServerInfo | None = Field(default=None, alias="serverInfo")


# =============================================================================
# LSP Request Params
# =============================================================================


class WorkspaceFolder(LSPModel):
    uri: str
    name: str


class ClientCapabilities(LSPModel, extra="allow"):
    pass


class InitializeParams(LSPModel):
    process_id: int | None = Field(alias="processId")
    root_uri: str | None = Field(alias="rootUri")
    root_path: str | None = Field(default=None, alias="rootPath")
    capabilities: ClientCapabilities
    workspace_folders: list[WorkspaceFolder] | None = Field(default=None, alias="workspaceFolders")
    initialization_options: Any | None = Field(default=None, alias="initializationOptions")
    trace: str | None = None


class ReferenceParams(TextDocumentPositionParams):
    context: ReferenceContext


class DocumentSymbolParams(LSPModel):
    text_document: TextDocumentIdentifier = Field(alias="textDocument")


class RenameParams(LSPModel):
    text_document: TextDocumentIdentifier = Field(alias="textDocument")
    position: Position
    new_name: str = Field(alias="newName")


class CallHierarchyItemParams(LSPModel):
    item: CallHierarchyItem


class TypeHierarchyItemParams(LSPModel):
    item: TypeHierarchyItem


class FileRename(LSPModel):
    old_uri: str = Field(alias="oldUri")
    new_uri: str = Field(alias="newUri")


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
