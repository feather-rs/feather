use super::*;

packets! {
    LoginStart {
        name String;
    }

    EncryptionResponse {
        shared_secret VarIntPrefixedVec<u8>;
        verify_token VarIntPrefixedVec<u8>;
    }

    LoginPluginResponse {
        message_id VarInt;
        successful bool;
        data LengthInferredVecU8;
    }
}
