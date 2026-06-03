" boilerplate template highlighting
" URL: https://github.com/casey/boilerplate
"
" :Boilerplate            highlight the current buffer as a boilerplate
"                         template, using the current filetype as the host
"                         language outside of blocks, and Rust inside blocks
" :Boilerplate {host}     same, but use {host} as the host language
" :Boilerplate            again to turn it back off

if exists("g:loaded_boilerplate")
  finish
endif
let g:loaded_boilerplate = 1

function! s:boilerplate(host) abort
  if &l:syntax ==# 'boilerplate'
    unlet! b:boilerplate_subtype
    let &l:syntax = &l:filetype
  else
    let b:boilerplate_subtype = a:host !=# '' ? a:host : &l:filetype
    setlocal syntax=boilerplate
  endif
endfunction

command! -nargs=? -complete=syntax Boilerplate call s:boilerplate(<q-args>)
