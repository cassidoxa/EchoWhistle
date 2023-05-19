ItemTextPointers:
        .ptr : fillbyte $FFFF : fill ($FF*2)+2
        .len : fillbyte $FFFF : fill ($FF*2)+2

table data/item_text.tbl,rtl
BoreasText: db "PROTO Boreas//"
WhistleText: db "PROTO Whistle//"
CrossBowText: db "PROTO Crossbow//"
LucaKeyText: db "PROTO Luca Key//"
GugnirText: db "PROTO Gugnir//"
HookText: db "PROTO Hook//"
cleartable

%ItemTextPointer(BoreasText, $B3, $0D)
%ItemTextPointer(GugnirText, $2A, $0D)
%ItemTextPointer(CrossBowText, $4E, $0F)
%ItemTextPointer(WhistleText, $ED, $0E)
%ItemTextPointer(LucaKeyText, $F3, $0F)
%ItemTextPointer(HookText, $FC, $0B)


WriteKeyItemText:
        REP #$20
        LDA.w CurrentSecret : AND.w #$00FF
        ASL : TAX
        LDA.w ItemTextPointers_len,X : TAY
        LDA.w ItemTextPointers_ptr,X : STA.b $20
        LDA.w #$0000 ; Zero out Acc high byte
        SEP #$20
        TYX
        -
                LDA.b ($20),Y : STA.l $7E2500,X
                DEX : DEY
        BPL -
        STZ.b $20 : STZ.b $21
RTS
