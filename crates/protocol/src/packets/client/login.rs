use super::*;

packets! {
    LoginStart {
        name String;
    }

    EncryptionResponse {
        shared_secret LengthPrefixedVec<u8>;
        verify_token LengthPrefixedVec<u8>;
    }

    LoginPluginResponse {
        message_id VarInt;
        successful bool;
        data LengthInferredVecU8;
    }
}
