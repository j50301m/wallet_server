#!/bin/bash

# Set your gRPC service details
PROTO_FILE="/Users/jason/kgs_lib/protos/protos/wallet/player_wallet.proto"
SERVICE_METHOD="player_wallet.PlayerWallet.Deposit"
SERVICE_ADDRESS="localhost:1691"

# Set test parameters
TOTAL_REQUESTS=1000
CONCURRENCY=40

# Run the test
ghz --insecure \
    --proto $PROTO_FILE \
    --call $SERVICE_METHOD \
    -d '
    {
    "action": 3,
    "amount": "100",
    "client_id": "7135148985370546176",
    "currency": "USD",
    "player_id": "7188781879867346945",
    "rollover_rate": "1",
    "transaction_id": "1",
    "wallet_source_id": "2"
}
' \
    -n $TOTAL_REQUESTS \
    -c $CONCURRENCY \
    $SERVICE_ADDRESS