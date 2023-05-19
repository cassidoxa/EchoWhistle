; Give us a "fake" NMI during menus

WaitForFakeNMI:
        LDA.w $0203 : PHA ; Preserve current NMI vector
        LDY.w $0201 : PHY ;
        LDA.b #FakeNMI>>16 : STA.w $0203
        LDY.w #FakeNMI : STY.w $0201
        LDA.b #$81 : STA.l $004200
        -
        LDA.w FakeNMIComplete : BEQ -
        STZ.w FakeNMIComplete
        LDA.b #$01 : STA.l $004200
        PLY : STY.w $0201
        PLA : STA.w $0203
RTL

FakeNMI:
        INC.w FakeNMIComplete
        RTI
