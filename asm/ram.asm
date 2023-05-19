;------------------------------------------------------------------------------
; RAM Labels and Assignments
;------------------------------------------------------------------------------

;------------------------------------------------------------------------------
; Bank $7E Mirrored WRAM
; $7E16C0-$7E16FF reserved
;------------------------------------------------------------------------------
EventCurrentOffset = $7E09D3         ; Offset into event data. Word length.
EventBytes = $7E09D5                 ; Event byte buffer. $3F bytes.
EventCurrentOpcode = $7E0A16         ;
CurrentSecretID = $7E1630            ; \ Written during secret receipt events.
CurrentSecretLocation = $7E1632      ; | Uses spell list RAM 
CurrentSecret = $7E1634              ; /
RxChannelPtr = $7E16C0               ; \ Five bit pointers into Rx and Tx buffers
TxChannelPtr = $7E16C2               ; / These are ring buffers. Top 3 bits get masked out.
ItemGivenFlag = $7E16C4              ; ¯\_(ツ)_/¯
ConnectionStatus = $7E16C8           ; $01 = Connected during title screen. Required for item
                                     ; service to work. Word length.
FakeNMIComplete = $7E16CA            ; $01 = Fake NMI complete
MapID = $7E1702                      ; Current map ID for non-overworld maps.
                                     ;
                                     ;
                                     ;
                                     ;


;------------------------------------------------------------------------------
; Bank $7F EchoWhistle RAM
;------------------------------------------------------------------------------
base $7FA000
SecretsBuffer: skip $20              ; Buffer for sensitive information. Word-length entries.
SecretsIDBuffer: skip $20            ; Contains a unique ID corresponding to entry above. Word-length entries.
RxCommand: skip $20                  ; \ Word-length commands. Tx is commands sent from SNES for client to
TxCommand: skip $20                  ; / execute and Rx command is client-to-SNES.
RxArgs: skip $40                     ; \ General purpose network ring buffers for passing messages.
TxArgs: skip $40                     ; | These channels contain two word-length arguments for a
                                     ; | command in the corresponding slot above.
                                     ; |
                                     ; |
                                     ; /
ReceiveStatus: skip $20              ; \ f - - - p a s o    | a = acknowledged | s = success | f = failure
                                     ;  |o = open for use   | p = pending
TransferStatus: skip $20             ; /
EchoMarkerWRAM: skip $14             ;
