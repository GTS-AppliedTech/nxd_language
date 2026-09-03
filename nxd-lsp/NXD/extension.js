const vscode = require('vscode');
const cp = require('child_process');
const path = require('path');

function activate(context) {
    const diagnostics =
        vscode.languages.createDiagnosticCollection('nxd');

    context.subscriptions.push(diagnostics);

    const validate = document => {
        if (document.languageId === 'nxd') {
            validateDocument(document, diagnostics);
        }
    };

    context.subscriptions.push(
        vscode.workspace.onDidOpenTextDocument(validate)
    );

    context.subscriptions.push(
        vscode.workspace.onDidChangeTextDocument(event => {
            validate(event.document);
        })
    );

    context.subscriptions.push(
        vscode.workspace.onDidSaveTextDocument(validate)
    );

    context.subscriptions.push(
        vscode.workspace.onDidCloseTextDocument(document => {
            diagnostics.delete(document.uri);
        })
    );

    for (const document of vscode.workspace.textDocuments) {
        validate(document);
    }
}

function validateDocument(document, diagnostics) {
    const source = document.getText();

    const workspaceRoot =
        vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;

    if (!workspaceRoot) {
        return;
    }

    const validatorPath = path.join(
        workspaceRoot,
        'lsp_validate.py'
    );

    const result = cp.spawnSync(
        'python',
        [validatorPath],
        {
            cwd: workspaceRoot,
            input: source,
            encoding: 'utf8',
            windowsHide: true
        }
    );
    // extension.js is:
    // nxd_language/nxd-lsp/NXD/extension.js
    //
    // lsp_validate.py is:
    // nxd_language/lsp_validate.py

    const problems = [];

    if (result.error) {
        problems.push(
            createDiagnostic(
                document,
                1,
                1,
                `NXD parser could not start: ${result.error.message}`,
                vscode.DiagnosticSeverity.Error
            )
        );

        diagnostics.set(document.uri, problems);
        return;
    }

    if (result.stderr && result.stderr.trim()) {
        problems.push(
            createDiagnostic(
                document,
                1,
                1,
                `NXD parser error: ${result.stderr.trim()}`,
                vscode.DiagnosticSeverity.Error
            )
        );

        diagnostics.set(document.uri, problems);
        return;
    }

    try {
        const response = JSON.parse(result.stdout.trim());

        for (const item of response.diagnostics) {
            const severity =
                item.severity === 'warning'
                    ? vscode.DiagnosticSeverity.Warning
                    : vscode.DiagnosticSeverity.Error;

            problems.push(
                createDiagnostic(
                    document,
                    item.line,
                    item.column,
                    item.message,
                    severity
                )
            );
        }
    } catch (error) {
        problems.push(
            createDiagnostic(
                document,
                1,
                1,
                `Invalid response from NXD parser: ${error.message}`,
                vscode.DiagnosticSeverity.Error
            )
        );
    }

    diagnostics.set(document.uri, problems);
}

function createDiagnostic(
    document,
    sourceLine,
    sourceColumn,
    message,
    severity
) {
    // NXD positions are one-based.
    // VS Code positions are zero-based.
    const requestedLine = Math.max(0, sourceLine - 1);
    const requestedColumn = Math.max(0, sourceColumn - 1);

    const line = Math.min(
        requestedLine,
        Math.max(0, document.lineCount - 1)
    );

    const lineLength = document.lineAt(line).text.length;

    const startColumn = Math.min(
        requestedColumn,
        lineLength
    );

    const endColumn = Math.min(
        startColumn + 1,
        lineLength
    );

    const diagnostic = new vscode.Diagnostic(
        new vscode.Range(
            line,
            startColumn,
            line,
            endColumn
        ),
        message,
        severity
    );

    diagnostic.source = 'NXD Parser';
    diagnostic.code = 'parser-error';

    return diagnostic;
}

function deactivate() {}

module.exports = {
    activate,
    deactivate
}