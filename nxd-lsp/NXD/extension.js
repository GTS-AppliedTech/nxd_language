const vscode = require('vscode');

function activate(context) {
    vscode.window.showInformationMessage(
        "NXD Activated Successfully"
    );

    return Promise.resolve();
}

function deactivate() {}

module.exports = {
    activate,
    deactivate
};