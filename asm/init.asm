InitRAMExpanded:
        JSL.l InitRam ; What we wrote over
        ; Initialize request statuses
        REP #$20
        LDX.w #$001E
        LDA.w #$0001
        -
                STA.l TransferStatus,X
                DEX #2
        BPL -
        STZ.w ItemGivenFlag
        ; Initialize Rx/Tx pointer
        STZ.w RxChannelPtr : STZ.w TxChannelPtr
        
        ; Initialize starting item request
        LDA.w #$0001 : STA.l TxCommand
        LDA.w #$0005 : STA.l TxArgs
        LDA.w #$0000 : STA.l TxArgs+$02
        LDA.w #$0008 : STA.l TransferStatus
        INC.w TxChannelPtr : INC.w TxChannelPtr

        ; Initialize marker. The client uses this to determine if we're good to read WRAM or not
        LDX.w #$0012
        -
                LDA.l EchoMarker,X : STA.l EchoMarkerWRAM,X
                DEX #2
        BPL -
        SEP #$20
RTL

EchoMarker:
db "Echo Whistle        "

SetFastROM:
        LDA.b #$01 : STA.w $420D
        STZ.w $420B ; What we wrote over
RTL
