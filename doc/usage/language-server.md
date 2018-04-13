# Using the language server

reproto comes with a [language server] to improve your developer experience.

This currently supports:

 * Contextual completions for types and packages.
 * Jump to definitions.

[language server]: https://langserver.org/

## Visual Studio Code

It is recommended that you use the [`reproto` extension], which will use the language server if it
can locate a reproto installation that is recent enough.

![completion in vscode](ls-completion.png?raw=true "completion in vscode")

[`reproto` extension]: https://marketplace.visualstudio.com/items?itemName=udoprog.reproto

## Neovim

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
