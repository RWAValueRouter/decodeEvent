use base64;
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
    // 输入的 base64 编码字符串
    let base64_data = "4HeX6/luJo8QJwAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAIAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAAAAAAAAAAAAAAAAAAAAD0IAalMSCSFmt2Evi8hMFVx5aGShzYBAAAAAAAAAAAAAAAAAFUAAAB7ImRlc3RDaGFpbk5hbWUiOiJPU01PU0lTIiwicmVjaXBpZW50Ijoib3NtbzE4NXlxcjIydmZxamd0eGthc2psejdnZnMyNGM3dGd2ajNtZDh4cyJ9";

    // 1. Base64 解码
    let decoded_bytes = base64::decode(base64_data).expect("Base64 decoding failed");

    // 2. 去掉前 8 个字节
    let stripped_bytes = &decoded_bytes[8..];

    // 3. 解析结构体
    let event = SwapAndBridgeEvent::from_bytes(&stripped_bytes);

    // 输出解析后的结果
    println!("{:#?}", event);
}

