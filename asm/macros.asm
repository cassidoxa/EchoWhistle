macro SecretsPointerEntry(label, id)
pushpc
        org SecretsPointerTable+(<id>*2)
        dw <label>
pullpc
endmacro

macro SecretsIDEntry(label, id, secret_id)
pushpc
        org SecretsIDTable+(<id>*2)
        <label>SecretID:
        dw <secret_id>
pullpc
endmacro

macro ItemTextPointer(item, id, len)
pushpc
        org ItemTextPointers_ptr+(<id>*2)
        dw <item>
        org ItemTextPointers_len+(<id>*2)
        dw <len>
pullpc
endmacro
