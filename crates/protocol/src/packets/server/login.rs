use super::*;

packets! {
    DisconnectLogin {
        reason String;
    }

    EncryptionRequest {
        server_id String;
        public_key LengthPrefixedVec<u8>;
        verify_token LengthPrefixedVec<u8>;
    }

    LoginSuccess {
        uuid Uuid;
        username String;
    }

    SetCompression {
        threshold VarInt;
    }

    LoginPluginRequest {
        message_id VarInt;
        successful bool;
        data LengthInferredVecU8;
    }
}
