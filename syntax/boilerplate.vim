" Vim syntax file
" Language:    boilerplate template
" URL:         https://github.com/casey/boilerplate

if exists("b:current_syntax")
  finish
endif

if !exists("main_syntax")
  let main_syntax = 'boilerplate'
endif

if !exists("b:boilerplate_subtype")
  let b:boilerplate_subtype = ''
endif

if b:boilerplate_subtype != '' && b:boilerplate_subtype !=? 'boilerplate'
  exe "runtime! syntax/" . b:boilerplate_subtype . ".vim"
  unlet! b:current_syntax
endif

syn include @rustTop syntax/rust.vim
unlet! b:current_syntax

syn cluster boilerplateRegions contains=boilerplateCode,boilerplateCodeLine,boilerplateInterpolation,boilerplateInterpolationLine

syn region boilerplateCode              matchgroup=boilerplateDelimiter start="{%"   end="%}" contains=@rustTop containedin=ALLBUT,@boilerplateRegions keepend
syn region boilerplateInterpolation     matchgroup=boilerplateDelimiter start="{{"   end="}}" contains=@rustTop containedin=ALLBUT,@boilerplateRegions keepend
syn region boilerplateCodeLine          matchgroup=boilerplateDelimiter start="%%"   end="$"  contains=@rustTop containedin=ALLBUT,@boilerplateRegions keepend oneline
syn region boilerplateInterpolationLine matchgroup=boilerplateDelimiter start="\$\$" end="$"  contains=@rustTop containedin=ALLBUT,@boilerplateRegions keepend oneline

hi def link boilerplateDelimiter PreProc

let b:current_syntax = 'boilerplate'

if main_syntax == 'boilerplate'
  unlet main_syntax
endif
