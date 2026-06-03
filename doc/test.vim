" Automated test for the boilerplate template highlighting plugin.
"
" Run headless:
"
"   nvim --headless -u doc/test.vim
"
" Exits 0 on success, 1 on failure, printing any failures.

set nocompatible
execute 'set runtimepath^=' . getcwd()
syntax on
runtime plugin/boilerplate.vim

function! s:name(l, c) abort
  return synIDattr(synID(a:l, a:c, 1), 'name')
endfunction

function! s:stack(l, c) abort
  return map(synstack(a:l, a:c), 'synIDattr(v:val, "name")')
endfunction

function! s:has(stack, pattern) abort
  return !empty(filter(copy(a:stack), 'v:val =~# a:pattern'))
endfunction

" A boilerplate block of every kind embedded in an HTML host.
let s:lines = [
\ '<body>',
\ '  {% if self.x { %}',
\ '  Hello {{ self.name }}!',
\ '  %% let y = 1;',
\ '  Byte $$ self.byte',
\ '  {% } %}',
\ '</body>',
\ ]

enew
call setline(1, s:lines)
set filetype=html
Boilerplate

call assert_equal('boilerplate', &syntax, 'syntax is boilerplate after toggle on')
call assert_equal('html', get(b:, 'boilerplate_subtype', ''), 'host captured from filetype')

" Host language highlights the text outside of blocks.
call assert_match('^html', s:name(1, 2), 'open tag is host html')
call assert_match('^html', s:name(7, 2), 'close tag is host html')
call assert_false(s:has(s:stack(1, 2), '^\(rust\|boilerplate\)'), 'no rust/boilerplate outside blocks')

" Every delimiter is a boilerplate delimiter.
for [s:l, s:c] in [[2, 3], [2, 4], [2, 18], [3, 9], [3, 22], [4, 3], [5, 8], [6, 8]]
  call assert_equal('boilerplateDelimiter', s:name(s:l, s:c),
\   printf('delimiter at %d,%d', s:l, s:c))
endfor

" Code block `{% ... %}` contains Rust.
call assert_true(s:has(s:stack(2, 6), 'boilerplateCode'), 'code block region')
call assert_match('^rust', s:name(2, 6), 'if is rust inside code block')

" Interpolation block `{{ ... }}` contains Rust.
call assert_true(s:has(s:stack(3, 13), 'boilerplateInterpolation'), 'interpolation region')
call assert_match('^rust', s:name(3, 13), 'self is rust inside interpolation')

" Code line `%% ...` contains Rust.
call assert_true(s:has(s:stack(4, 6), 'boilerplateCodeLine'), 'code line region')
call assert_match('^rust', s:name(4, 6), 'let is rust inside code line')

" Interpolation line `$$ ...`, recognized mid-line, contains Rust.
call assert_true(s:has(s:stack(5, 11), 'boilerplateInterpolationLine'), 'interpolation line region')
call assert_match('^rust', s:name(5, 11), 'self is rust inside interpolation line')

" Toggling off restores the host syntax.
Boilerplate
call assert_equal('html', &syntax, 'syntax restored to host after toggle off')
call assert_false(s:has(s:stack(2, 6), '^\(rust\|boilerplate\)'), 'no rust/boilerplate after toggle off')
call assert_notequal('boilerplateDelimiter', s:name(2, 3), 'delimiter unhighlighted after toggle off')
call assert_false(exists('b:boilerplate_subtype'), 'subtype cleared after toggle off')

" An explicit host argument overrides the filetype.
Boilerplate markdown
call assert_equal('markdown', get(b:, 'boilerplate_subtype', ''), 'explicit host argument')
call assert_equal('boilerplateDelimiter', s:name(2, 3), 'blocks still highlight with custom host')

if empty(v:errors)
  qall!
else
  for s:error in v:errors
    echomsg s:error
  endfor
  cquit!
endif
