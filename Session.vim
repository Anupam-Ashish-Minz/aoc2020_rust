let SessionLoad = 1
let s:so_save = &g:so | let s:siso_save = &g:siso | setg so=0 siso=0 | setl so=-1 siso=-1
let v:this_session=expand("<sfile>:p")
silent only
cd ~/work/codingChallenges/aoc/aoc2020_rust
if expand('%') == '' && !&modified && line('$') <= 1 && getline(1) == ''
  let s:wipebuf = bufnr('%')
endif
set shortmess=aoO
badd +217 src/day8.rs
badd +15 src/day9.rs
badd +1 src/day7.rs
badd +1 src/main.rs
badd +1 .git/index
badd +0 .gitignore
argglobal
%argdel
edit src/day9.rs
set splitbelow splitright
wincmd _ | wincmd |
split
1wincmd k
wincmd w
set nosplitright
wincmd t
set winminheight=0
set winheight=1
set winminwidth=0
set winwidth=1
exe '1resize ' . ((&lines * 20 + 22) / 44)
exe '2resize ' . ((&lines * 21 + 22) / 44)
argglobal
setlocal fdm=manual
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=1
setlocal fml=1
setlocal fdn=5
setlocal fen
silent! normal! zE
9,20fold
21,34fold
35,45fold
46,67fold
74,94fold
72,94fold
96,102fold
104,111fold
68,111fold
let s:l = 15 - ((6 * winheight(0) + 10) / 20)
if s:l < 1 | let s:l = 1 | endif
exe s:l
normal! zt
15
normal! 048|
lcd ~/work/codingChallenges/aoc/aoc2020_rust/src
wincmd w
argglobal
if bufexists("~/work/codingChallenges/aoc/aoc2020_rust/.git/index") | buffer ~/work/codingChallenges/aoc/aoc2020_rust/.git/index | else | edit ~/work/codingChallenges/aoc/aoc2020_rust/.git/index | endif
if &buftype ==# 'terminal'
  silent file ~/work/codingChallenges/aoc/aoc2020_rust/.git/index
endif
setlocal fdm=manual
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=9
setlocal fml=1
setlocal fdn=5
setlocal fen
let s:l = 1 - ((0 * winheight(0) + 10) / 21)
if s:l < 1 | let s:l = 1 | endif
exe s:l
normal! zt
1
normal! 0
lcd ~/work/codingChallenges/aoc/aoc2020_rust/.git
wincmd w
2wincmd w
exe '1resize ' . ((&lines * 20 + 22) / 44)
exe '2resize ' . ((&lines * 21 + 22) / 44)
tabnext 1
if exists('s:wipebuf') && getbufvar(s:wipebuf, '&buftype') isnot# 'terminal'
  silent exe 'bwipe ' . s:wipebuf
endif
unlet! s:wipebuf
set winheight=1 winwidth=20 winminheight=1 winminwidth=1 shortmess=filnxtToOFAc
let s:sx = expand("<sfile>:p:r")."x.vim"
if filereadable(s:sx)
  exe "source " . fnameescape(s:sx)
endif
let &g:so = s:so_save | let &g:siso = s:siso_save
let g:this_session = v:this_session
let g:this_obsession = v:this_session
doautoall SessionLoadPost
unlet SessionLoad
" vim: set ft=vim :
