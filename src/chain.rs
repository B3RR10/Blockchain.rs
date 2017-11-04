
struct Blockchain {}

impl Blockchain {
    fn new() {}
    fn addBlock() {}
    fn genesisBlock() {}
    fn iterator() {}
    fn findUnspentTransaction() {}
    fn findUTXO() {}
    fn findSpandableOutputs() {}
    fn mineBlock() {}
    fn findTransaction() {}
    fn signTransaction() {}
    fn verifyTransaction() {}
}

struct BlockchainIterator {}

impl BlockchainIterator {
    fn new() {}
}

struct Transaction {}

impl Transaction {
    fn setId() {}
    fn sign() {}
    fn trimmedCopy() {}
    fn verify() {}
}

struct TXInput {}

impl TXInput {
    fn canUnlinkOutputWith() {}
    fn usesKey() {}
}

struct TXOutput {}

impl TXOutput {
    fn canBeUnlockedWith() {}
    fn locked() {}
    fn isLockedWithKey() {}
}

struct Wallet {}

impl Wallet {
    fn new() {}
    fn getAddress() {}
}

struct Wallets {}

fn newUTXOTransaction() {}

fn newKeyPair() {}

fn hashPubKey() {}

fn checksum() {}


impl UTXOSet {
    fn reindex() {}
    fn findSpandableOutputs() {}
    fn findUTXO() {}
    fn update() {}
}

struct MerkleTree {}

impl MerkleTree {
    fn new() {}
}

struct MerkleTreeNode {}

impl MerkleTreeNode {
    fn new() {}
}
