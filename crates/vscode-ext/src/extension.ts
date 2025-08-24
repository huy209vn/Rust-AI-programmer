import * as vscode from 'vscode';
export function activate(ctx: vscode.ExtensionContext) {
  const disposable = vscode.commands.registerCommand('rusta.explain', async () => {
    vscode.window.showInformationMessage('Rusta: Explain (daemon not wired yet)');
  });
  ctx.subscriptions.push(disposable);
}
export function deactivate() {}
