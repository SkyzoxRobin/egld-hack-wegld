PROXY="https://devnet-gateway.elrond.com"
CHAIN="D"

OWNER="../../wallet-owner.pem"
CONTRACT="output/egld-hack.wasm"
SC_ADDRESS="erd1qqqqqqqqqqqqqpgqw5yhn64dc9hv5yzmqq8wzt660nqy6tqznqjsrqw20g"

INITIAL_CALLER="$(erdpy wallet pem-address $OWNER)"
INITIAL_CALLER_HEX="0x$(erdpy wallet bech32 --decode ${INITIAL_CALLER})"
MAIN_CONTRACT_HEX="0x$(erdpy wallet bech32 --decode erd1qqqqqqqqqqqqqpgqqjqt205yvzspqp9zvgs6e744spd5acwynqjsq50vqn)"

deploy() {
    erdpy --verbose contract deploy --bytecode="$CONTRACT" --recall-nonce \
        --pem=$OWNER \
        --gas-limit=599000000 \
        --proxy=$PROXY --chain=$CHAIN \
        --arguments $INITIAL_CALLER_HEX $MAIN_CONTRACT_HEX \
        --outfile="deploy-devnet.interaction.json" --send || return

    TRANSACTION=$(erdpy data parse --file="deploy-devnet.interaction.json" --expression="data['emittedTransactionHash']")
    ADDRESS=$(erdpy data parse --file="deploy-devnet.interaction.json" --expression="data['contractAddress']")

    erdpy data store --key=address-devnet --value=${ADDRESS}
    erdpy data store --key=deployTransaction-devnet --value=${TRANSACTION}

    echo "Smart contract address: ${ADDRESS}"
}

wrapEgld() {
    WRAP_CONTRACT_HEX="0x$(erdpy wallet bech32 --decode erd1qqqqqqqqqqqqqpgq7ykazrzd905zvnlr88dpfw06677lxe9w0n4suz00uh)"
    erdpy --verbose contract call ${ADDRESS} --recall-nonce \
    --pem=$OWNER \
    --gas-limit=150000000 \
    --value 100000 \
    --proxy=$PROXY --chain=$CHAIN \
    --function="wrapEgld" \
    --arguments $WRAP_CONTRACT_HEX \
    --send || return
}

withdraw() {
    MAIN_CONTRACT="erd1qqqqqqqqqqqqqpgqqjqt205yvzspqp9zvgs6e744spd5acwynqjsq50vqn"
    erdpy --verbose contract call ${MAIN_CONTRACT} --recall-nonce \
    --pem=$OWNER \
    --gas-limit=150000000 \
    --proxy=$PROXY --chain=$CHAIN \
    --function="withdraw" \
    --send || return
}

wrapEgld() {
    WRAP_CONTRACT="erd1qqqqqqqqqqqqqpgq7ykazrzd905zvnlr88dpfw06677lxe9w0n4suz00uh"
    erdpy --verbose contract call ${WRAP_CONTRACT} --recall-nonce \
    --pem=$OWNER \
    --value 10000000000000000000000 \
    --gas-limit=150000000 \
    --proxy=$PROXY --chain=$CHAIN \
    --function="wrapEgld" \
    --send || return
}

