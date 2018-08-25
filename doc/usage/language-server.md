# Using the language server

reproto comes with a built-in [language server] that greatly enhances your developer experience.

*This is work in progress!*, you can follow the progress of this in [issue #34].
Feel free to voice your opinions there, or pitch in if you want to give a hand!

The following editors are officially supported:

 * [Visual Studio Code](#visual-studio-code)
 * [Neovim](#neovim)

Next up we will go through the features which are available in the language server through
[Visual Studio Code](#visual-studio-code).

Since a picture is worth a thousand words, gifs should be priceless ;)

[issue #34]: https://github.com/reproto/reproto/issues/34

## As-you-type diagnostics and feedback

The reproto compiler is _fast_.
This allows us to provide diagnostics at the speed that you are typing.
Well, maybe unless you are [one of these people].

This gives us error messages highligting the location responsible for the error and hints at how
you can fix them.

![diagnostics](ls-diagnostics.gif?raw=true "diagnostics in vscode")

[one of these people]: https://www.youtube.com/watch?v=m9EXEpjSDEw

## Jump to definitions (`CTRL+Click`)

We can jump to any definitions, to files _anywhere_ in your path.

![jump to definitions](ls-jump-to-definitions.gif?raw=true "jump to definitions in vscode")

## Contextual completions (`CTRL+ENTER`)

The language server uses the same compiler infrastructure as the command line tool, so it can
inxpect the full context of the language to provide a rich set of completions.

![completions](ls-completions.gif?raw=true "completions in vscode")

## Rename package prefixes (`F2`)

We can rename package prefixes for [imports].
This will replace the prefix on any location where it is used.

Since package prefixes are isolated to a file, this change will only affect the current file.

If you attempt to rename an implicitly prefix import like `ex.github.gists`, a new alias will be
introduced.

![renaming package prefixes](ls-rename-package.gif?raw=true "renaming package in vscode")

[imports]: ../spec.md#imports
[language server]: https://langserver.org/

## Rename types (`F2`)

You can rename types locally, or even across packages.

Packages imported from the repository are marked read-only, and cannot be modified through
refactoring.

![renaming types](ls-rename-types.gif?raw=true "renaming types in vscode")

## Go to workspace symbols (`CTRL+T`)

You can use the built-in search functionality to quickly jump to any symbol in the workspace.

![go to workspace symbols](ls-workspace-symbols.gif?raw=true "go to workspace symbols in vscode")

## Go to file symbol (`CTRL+SHIFT+O`)

Similarly you can use the built-in go to file symbols to quickly jump back and forth between
symbols in a single file.

![go to file symbols](ls-file-symbols.gif?raw=true "go to file symbols in vscode")

## Find all references (`SHIFT+F12`)

We can find all references to a type, allowing us insights into how and where it is used in a given
project.

![find references](ls-references.gif?raw=true "find references in vscode")

# Visual Studio Code

It is recommended that you use the [`reproto` extension].

The extension is capable of installing reproto automatically if it's not already installed.

You can kick this off by doing `CTRL+SHIFT+P`, and running `Reproto: initialize new project`.

![initialize in vscode](ls-initialize.png?raw=true "initialize in vscode")

[`reproto` extension]: https://marketplace.visualstudio.com/items?itemName=udoprog.reproto

# Neovim

You can plug the language server into [LanguageClient-neovim] using the following configuration:

```vim
function! SetupReproto()
  nnoremap <silent> gd :call LanguageClient_textDocument_definition()<CR>
endfunction

autocmd BufNewFile,BufRead *.reproto :call SetupReproto()

let g:LanguageClient_serverCommands = {
    \ 'reproto': ['reproto', 'language-server'],
    \ }

let g:LanguageClient_rootMarkers = {
    \ 'reproto': ['reproto.toml'],
    \ }
```

This will allow you to jump to definitions by typing `gd`.

[LanguageClient-neovim]: https://github.com/autozimu/LanguageClient-neovim
