use alloy_primitives::{address, map::HashMap, Address};
use reth_revm::state::Account;

pub(crate) fn fix_state_diff(
    block_number: u64,
    tx_index: usize,
    changes: &mut HashMap<Address, Account>,
) {
    // Improper self destructs
    const TX_LIST: [(u64, usize, Address); 18] = [
        (1467569, 0, address!("0x33f6fe38c55cb100ce27b3138e5d2d041648364f")),
        (1467631, 0, address!("0x33f6fe38c55cb100ce27b3138e5d2d041648364f")),
        (1499313, 2, address!("0xe27bfc0a812b38927ff646f24af9149f45deb550")),
        (1499406, 0, address!("0xe27bfc0a812b38927ff646f24af9149f45deb550")),
        (1499685, 0, address!("0xfee3932b75a87e86930668a6ab3ed43b404c8a30")),
        (1514843, 0, address!("0x723e5fbbeed025772a91240fd0956a866a41a603")),
        (1514936, 0, address!("0x723e5fbbeed025772a91240fd0956a866a41a603")),
        (1530529, 2, address!("0xa694e8fd8f4a177dd23636d838e9f1fb2138d87a")),
        (1530622, 2, address!("0xa694e8fd8f4a177dd23636d838e9f1fb2138d87a")),
        (1530684, 3, address!("0xa694e8fd8f4a177dd23636d838e9f1fb2138d87a")),
        (1530777, 3, address!("0xa694e8fd8f4a177dd23636d838e9f1fb2138d87a")),
        (1530839, 2, address!("0x692a343fc401a7755f8fc2facf61af426adaf061")),
        (1530901, 0, address!("0xfd9716f16596715ce765dabaee11787870e04b8a")),
        (1530994, 3, address!("0xfd9716f16596715ce765dabaee11787870e04b8a")),
        (1531056, 4, address!("0xdc67c2b8349ca20f58760e08371fc9271e82b5a4")),
        (1531149, 0, address!("0xdc67c2b8349ca20f58760e08371fc9271e82b5a4")),
        (1531211, 3, address!("0xdc67c2b8349ca20f58760e08371fc9271e82b5a4")),
        (1531366, 1, address!("0x9a90a517d27a9e60e454c96fefbbe94ff244ed6f")),
    ];
    if block_number < 1467569 || block_number > 1531366 {
        return;
    }
    for (block_num, idx, address) in TX_LIST {
        if block_number == block_num && tx_index == idx {
            changes.remove(&address);
        }
    }
}
