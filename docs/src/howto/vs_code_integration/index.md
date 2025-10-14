# VS Code Integration

There are certainly different ways to do this, but here we provide one way that has been already
successuflly tested by our community.

This solution is based on the [LaTeX Workshop](https://marketplace.visualstudio.com/items?itemName=James-Yu.latex-workshop) extension that you can install for free from the VS Code Marketplace.

For further details on its configuration please, refer to the upstream [documentation](https://github.com/James-Yu/LaTeX-Workshop/wiki).

At the root of your VS Code project / workspace create a file named
`.vscode/settings.json` with the following content:

```json
{
    "latex-workshop.latex.recipe.default": "tectonic",
    "latex-workshop.latex.autoBuild.run": "onSave",
    "latex-workshop.latex.outDir": "%WORKSPACE_FOLDER%/build/",
    "latex-workshop.view.pdf.viewer": "tab",
    "latex-workshop.latex.recipes": [
        {
            "name": "tectonic",
            "tools": [
                "tectonic"
            ]
        }
    ],
    "latex-workshop.latex.tools": [
        {
            "name": "tectonic",
            "command": "tectonic",
            "args": [
                "-X",
                "build",
                "--keep-intermediates",
                "--keep-logs"
            ],
        }
    ],
    "latex-workshop.formatting.latex": "latexindent",
}
```

This assumes that both `tectonic` and `latexindent` are available from your PATH
environment variable.

Any time you save any of the .tex files in your project you should see the output
under the `build` folder depending on your Tectonic configuration.