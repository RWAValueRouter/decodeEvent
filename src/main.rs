use std::str;
use std::convert::TryInto;

// 定义结构体
#[derive(Debug)]
struct SwapAndBridgeEvent {
    bridge_usdc_amount: u64,
    buy_token: [u8; 32],              // Pubkey
    guaranteed_buy_amount: Vec<u8>,
    dest_domain: u32,
    recipient: [u8; 32],              // Pubkey
    bridge_nonce: u64,
    swap_nonce: u64,
    memo: String,                     // memo is a string encoded in JSON
}

// 将字节数组解析为 SwapAndBridgeEvent 结构体
impl SwapAndBridgeEvent {
    fn from_bytes(data: &[u8]) -> Self {
        // 解析 bridge_usdc_amount (u64 - 8 bytes)
        let bridge_usdc_amount = u64::from_le_bytes(data[0..8].try_into().unwrap());

        // 解析 buy_token (Pubkey - 32 bytes)
        let buy_token = data[8..40].try_into().unwrap();

        // 解析 guaranteed_buy_amount (Vec<u8> - 4 bytes length + bytes)
        let guaranteed_buy_amount_len = u32::from_le_bytes(data[40..44].try_into().unwrap()) as usize;
        let guaranteed_buy_amount = data[44..44 + guaranteed_buy_amount_len].to_vec();

        // 解析 dest_domain (u32 - 4 bytes)
        let dest_domain = u32::from_le_bytes(data[44 + guaranteed_buy_amount_len..48 + guaranteed_buy_amount_len].try_into().unwrap());

        // 解析 recipient (Pubkey - 32 bytes)
        let recipient = data[48 + guaranteed_buy_amount_len..80 + guaranteed_buy_amount_len].try_into().unwrap();

        // 解析 bridge_nonce (u64 - 8 bytes)
        let bridge_nonce = u64::from_le_bytes(data[80 + guaranteed_buy_amount_len..88 + guaranteed_buy_amount_len].try_into().unwrap());

        // 解析 swap_nonce (u64 - 8 bytes)
        let swap_nonce = u64::from_le_bytes(data[88 + guaranteed_buy_amount_len..96 + guaranteed_buy_amount_len].try_into().unwrap());

        // 解析 memo (Vec<u8> - 4 bytes length + bytes)
        let memo_len = u32::from_le_bytes(data[96 + guaranteed_buy_amount_len..100 + guaranteed_buy_amount_len].try_into().unwrap()) as usize;
        let memo_bytes = &data[100 + guaranteed_buy_amount_len..100 + guaranteed_buy_amount_len + memo_len];
        let memo = str::from_utf8(memo_bytes).unwrap().to_string();

        SwapAndBridgeEvent {
            bridge_usdc_amount,
            buy_token,
            guaranteed_buy_amount,
            dest_domain,
            recipient,
            bridge_nonce,
            swap_nonce,
            memo,
        }
    }
}

fn main() {
    // 输入的 hex 数据
    let hex_data = "10270000000000000000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000003d0801a94c4824859add84be2f21305571e5a19287360100000000000000000000000000550000007b2264657374436861696e4e616d65223a224f534d4f534953222c22726563697069656e74223a226f736d6f31383579717232327666716a6774786b61736a6c7a37676673323463377467766a336d64387873227d";

    // 将 hex 转换为字节数组
    let bytes = hex::decode(hex_data).expect("Decoding failed");

    // 解析结构体
    let event = SwapAndBridgeEvent::from_bytes(&bytes);

    // 输出解析后的结果
    println!("{:#?}", event);
}

