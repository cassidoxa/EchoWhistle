; Asar v1.18

org $6FFFFF ; Expand to 3mb
org $00E967 : fillbyte $FF : fill $11 ; Unused 2-opcode event command

incsrc macros.asm
incsrc hooks.asm
incsrc ram.asm
incsrc vanillalabels.asm


org $B58000
incsrc itemservice.asm
incsrc init.asm
incsrc secrets.asm
incsrc text.asm
incsrc nmi_menu.asm
