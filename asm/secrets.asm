
MapSecretsTable:
; Table is two word entries per secret. First is secret ID and second is requested secrets buffer
; slot.
fillbyte $FF : fill $400

AntlionRoomSecrets:
dw $0000, $0000
dw $FFFF
FabulThroneRoomSecrets:
dw $0001, $0000
dw $FFFF
OrdealsSummitRoomSecrets:
dw $0002, $0000
dw $FFFF
BaronInnRoomSecrets:
dw $0003, $0000
dw $FFFF
ToroiaCastleSecrets:
dw $0004, $0000
dw $FFFF
StartingSecrets:
dw $0005, $0000
dw $FFFF

SecretsPointerTable:
fillbyte $FF : fill $400 
%SecretsPointerEntry(AntlionRoomSecrets, $79)
%SecretsPointerEntry(FabulThroneRoomSecrets, $49)
%SecretsPointerEntry(OrdealsSummitRoomSecrets, $15)
%SecretsPointerEntry(BaronInnRoomSecrets, $0B)
%SecretsPointerEntry(ToroiaCastleSecrets, $58)
%SecretsPointerEntry(StartingSecrets, $2A)

SecretsIDTable:
fillbyte $FFFF : fill $100*2 
%SecretsIDEntry(AntlionRoom, $21, $00)
%SecretsIDEntry(FabulThroneRoom, $22, $01)
%SecretsIDEntry(OrdealsSummitRoom, $23, $02)
%SecretsIDEntry(BaronInnRoom, $24, $03)
%SecretsIDEntry(ToroiaCastle, $26, $04)
%SecretsIDEntry(Starting, $20, $05)
