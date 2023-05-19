; SNES Commands
; 00 - None
; 01 - Clear Tx slot indicated by args
; 02 - Acknowledge connection, set ConnectionStatus
;
; Client Commands
; 00 - None
; 01 - Item/secret request
; 02 - Acknowledge connection
;

MapRequestOverworld:
        JSR.w SendMapSecretsRequest
        LDA.l $19FE69,X ; What we wrote over
RTL

MapRequestUnderworld:
        JSR.w SendMapSecretsRequest
        LDA.l $158303,X ; What we wrote over
RTL

MapRequestUnderworldCutscene:
        PHX
        JSR.w SendMapSecretsRequest
        PLX
        LDA.w $09D7,X : AND.b #$3F
RTL

SendMapSecretsRequest:
; TODO: currently only uses the first available slot and possibly forward slots without checking
; if they're available because right now we're only requesting one secret per map.
        PHX : PHB
        PHK : PLB
        REP #$20
        
        AND.w #$00FF : ASL : TAY
        LDA.w SecretsPointerTable,Y : CMP.w #$FFFF : BEQ .done
        TAY
        LDA.w TxChannelPtr : TAX
        -
                LDA.l TransferStatus,X : BIT.w #$0001 : BNE .goodslot
                        INX #2 : TXA : AND.w #$001F : TAX
        BRA -
        .goodslot
        LDA.w #$0001 : STA.l TxCommand,X
        PHX
        TXA : ASL : TAX
        -
                LDA.w $0000,Y : CMP.w #$FFFF : BEQ .continue
                        STA.l TxArgs,X
                        INY #2 : INX #2
                        LDA.w $0000,Y : STA.l TxArgs,X
                        INY #2 : INX #2
        BRA -
        ; Store this last so it can indicate the command is ready for reading
        .continue
        PLX
        LDA.w #$0008 : STA.l TransferStatus,X
        INX #2 : TXA : AND.w #$001F
        STA.w TxChannelPtr

        .done
        SEP #$20
        PLB : PLX

RTS

; TODO: Trim the register preservation
CheckKeyItemRequest:
        PHX : PHY : PHP : PHB
        PHK : PLB
        JSR.w WriteEventBytesTwo
        LDA.w EventBytes+$02 : CMP.b #$20 : BCC .char
        REP #$20
        LDA.w EventBytes+$02 : STA.w CurrentSecretLocation
        AND.w #$00FF : ASL : TAX
        LDA.w SecretsIDTable,X :  STA.w CurrentSecretID
        LDA.w EventBytes+$03 : AND.w #$00FF : ASL : TAX
        -
                LDA.l SecretsIDBuffer,X : CMP.w CurrentSecretID
        BNE - ; TODO
        LDA.l SecretsBuffer,X : STA.w $1622 : STA.w CurrentSecret
        SEP #$20
        JSR.w WriteKeyItemText
        PLB : PLP : PLY : PLX       
RTL

        .char ; hhack to get items working
        REP #$20
        LDA.w #$0002
        SEP #$20
        LDX.w #$0403 : STX.w $1622
        LDX.w #$0000
        STA.w $1670
        PHX : PHA
        JSR.w InitChar
        PLA : PLX
        STX.w $1670
        STZ.w $1672
        ASL : STA.w $1671
        ASL : CLC
        ADC.w $1671 : ADC.w #$8D80
        ADC.b ($16),Y
        LDA.w $1672 : ADC.w #$8D11 : ADC.b ($16)
        LDA.w #$8D7E : ADC.b ($16,S),Y : LDA.w #$8D06
        STZ.b $16,X
        JSL.l FillBufferChar

        SEP #$20
        PLB : PLP : PLY : PLX       
RTL


StoreSecretKeyItem:
        PHX : PHY : PHP : PHB
        PHK : PLB
        JSR.w WriteEventBytesOne
        LDX.w CurrentSecret : STX.w $09D8
        LDA.w $09D9
        PLB : PLP : PLY : PLX       
RTL

GiveSecretKeyItem:
        PHX : PHY : PHP : PHB
        PHK : PLB
        JSR.w WriteEventBytesOne
        LDA.w $09D7 : CMP.b #$20 : BCC +
                LDX.w CurrentSecret : STX.w $09D8 : STX.w $1622
                BRA ++
        +
        LDX.w #$0402 : STX.w $09D8 : STX.w $1622
        ++
        JSR.w SetLocationBit
        LDA.w $1623 : CMP.b #$04 : BEQ .notki
        JSR.w SetPlotFlags
        JSR.w SetKeyItemBit

        LDA.w $1680 : ASL : TAX
        LDA.w CurrentSecretLocation : STA.l $707080,X
        INX
        LDA.b #$00 : STA.l $707080,X
        STZ.b $06
        LDA.w CurrentSecret : STA.w $08FB
        LDX.w #$9818 : STX.w $0FFB
        PLB : PLP : PLY : PLX
JML.l $00FFB5
        
        .notki
        INC.w $1579
        LDA.w $129F
        AND.b #$BF
        STA.w $129F
        LDY.w #$0000
        -
                LDA.w $1000,Y
                AND.b #$1F
                CMP.w $1622
                BEQ .done
                JSR.w Cycle
                CPY.w #$0140
        BCC -
        .done
        PLB : PLP : PLY : PLX
RTL

Cycle:
        PHA
        PHP
        REP #$20
        TYA
        CLC
        ADC.w #$0040
        TAY
        LDA.w #$0000
        PLP
        PLA
RTS


SetLocationBit:
        LDA.w CurrentSecretLocation
        LDX.w #$1510
        PHA
        LSR #3
        .seek
                BEQ .next
                INX : DEC
        BRA .seek
        .next
        PLA : AND.b #$07 : XBA
        LDA #$00 : XBA : TAY
        LDA #$01
        -
                CPY #$0000
                BEQ .done
                ASL
                DEY
        BRA -
        .done
        ORA.w $0000,X
        STA.w $0000,X
RTS

SetPlotFlags: ; Not actually plot flags probably
        LDA.w CurrentSecret : STA.b $3D
        LDX.w #$0000
        LDA.l $209844,X : BEQ .done
        CMP.b $3D : BEQ +
                NOP ; TODO
        +
        INX
        LDA.l $209844,X
        PHA
        LSR #3
        STA.b $3D : STZ.b $3E
        PLA : AND.b #$07 : TAY
        LDA.b #$01
        -
                CPY.w #$0000 : BEQ +
                ASL : DEY
        BRA -
        LDX.b $3D
        ORA.w $1280,X
        STA.w $1280,X

        .done
RTS

SetKeyItemBit:
        LDA.w CurrentSecret : STA.w $1680
        LDX.w #$0000
        -
                LDA.l $20E21D,X : BEQ +
                CMP.w $1680 : BEQ +
                INX
        BRA -
        +
        TXA
        STA.w $1680
        LDX.w #$1500
        PHA
        LSR #3
        .seek
                BEQ .next
                INX : DEC
        BRA .seek
        .next
        PLA : AND.b #$07 : XBA
        LDA #$00 : XBA : TAY
        LDA #$01
        -
                CPY #$0000
                BEQ .done
                ASL
                DEY
        BRA -
        .done
        ORA.w $0000,X
        STA.w $0000,X
RTS


WriteEventBytesTwo:
; In: X - Current offset into event data block, the opcode we're executing plus two.
        LDY.w #$0002
        LDA.l EventData,X : STA.w EventBytes,Y
        INC.b $B3 : INX : INY
        LDA.l EventData,X : STA.w EventBytes,Y
        INC.b $B3 : INX : INY
        STX.w EventCurrentOffset
        LDA.b #$FF : STA.w EventBytes,Y
RTS

WriteEventBytesOne:
; In: X - Current offset into event data block, the opcode we're executing plus two.
        LDY.w #$0002
        LDA.l EventData,X : STA.w EventBytes,Y
        INC.b $B3 : INX : INY
        STX.w EventCurrentOffset
        LDA.b #$FF : STA.w EventBytes,Y
RTS

InitChar:
        STA.w $1619
        LDA.b #$00 : XBA
        LDA.w $1619 : ASL : STA.w $168E
        ASL : CLC : ADC.w $168E
        TAX
        LDA.w $1180,X
        ; BEQ + : JMP _ : +
        PHX : TXY : PHY
        LDA.w $1619 : XBA
        LDA.b #$00 : XBA
        TAX
        LDA.l $21F700,X : TAX
        PLY
        LDA.l $018456,X : ASL : STA.w $168E
        ASL : CLC : ADC.w $168E : TAX
        LDA.b #$06 : STA.w $168F
        -
        LDA.l $707000,X : STA.w $1180,Y
        INX : INY
        DEC.w $168F
        BNE -
        PLY
.done
RTS

FillBufferChar:
        LDA.b #$00 : XBA
        LDA.w $1670 : ASL #5 : TAX
        LDY.w $1671
        STZ.w $1675
        PHB
        LDA.w $1673
        PHA
        PLB
        -
        LDA.w $0000,Y : CMP.b #$15 : BNE +
                INY
                DEC.w $1674
        BNE -
        +
        -
        LDA.w $1675
        CMP.w $1674
        BEQ +
                INC.w $1675
                LDA.w $0000,Y
                STA.l $7E2500,X
                INY
                INX
        BRA -
        +
        LDA.b #$00
        STA.l $7E2500,X
        -
                LDA.w $1675
                BEQ .done
                DEX
                DEC.w $1675
                LDA.l $7E2500,X
                CMP.b #$FF
                BNE .done
                LDA.b #$00
                STA.l $7E2500,X
        BRA -
        .done
        PLB
RTL


pushpc

org $20F000
CheckKeyItemRequestLong:
        JSL.l CheckKeyItemRequest
RTL

StoreSecretKeyItemLong:
        JSL.l StoreSecretKeyItem
RTL

GiveSecretKeyItemLong:
        JSL.l GiveSecretKeyItem
        .done
RTL

pullpc
