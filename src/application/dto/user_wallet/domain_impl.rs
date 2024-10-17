use crate::application::dto::common::ToProtoTrait;

use crate::domain::*;

impl ToProtoTrait for UserWalletWithRollover {
    type ProtoType = protos::player_wallet::WalletModel;

    fn to_proto(self) -> Self::ProtoType {
        Self::ProtoType {
            user_id: self.user_id,
            client_id: self.client_id,
            currency: self.currency_name,
            wallet_source_id: self.wallet_source_id,
            wallet_source_name: self.wallet_source_name,
            amount: self.amount.to_string(),
            requirement_rollover: self.requirement_rollover.to_string(),
            achievement_rollover: self.achievement_rollover.to_string(),
        }
    }
}

impl ToProtoTrait for Vec<UserWalletWithRollover> {
    type ProtoType = protos::player_wallet::GetPlayerWalletListResponse;

    fn to_proto(self) -> Self::ProtoType {
        Self::ProtoType {
            wallet_list: self
                .into_iter()
                .map(|user_wallet| user_wallet.to_proto())
                .collect(),
        }
    }
}
