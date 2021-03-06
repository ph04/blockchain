use crate::transaction::Transaction;
use std::convert::TryInto;
use sha2::{Sha512, Digest};
use chrono::{DateTime, Utc};

/// A structure to handle blocks for the blockchain of the currency.
/// 
/// Every block of the chain contains:
/// - the index (the #0 block is the genesis block)
/// - the SHA-512 hash of the previous block
/// - the transactions of the block
/// (the number of transactions per block is set while generating the blockchain)
/// - the nonce, which is used for the proof of work
/// - the `DateTime<Utc>` time when the block was generated
/// - the hash of the block generated
#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    index: usize,
    prev_hash: [u8; 64],
    transactions: Vec<Transaction>,
    nonce: u128,
    time: DateTime<Utc>,
    hash: [u8; 64],
}

impl Block {
    /// Generates a new `Block`.
    /// 
    /// # Example
    /// ```
    /// # use blockchain::{block::Block, transaction::Transaction, account::Account};
    /// let mut glenn = Account::new("Glenn", "Paris", "glenn_paris_PassWord88");
    /// let william = Account::new("William", "Brown", "WilliamTheConqueror22");
    /// glenn.add_money(20.0);
    /// 
    /// let transaction = Transaction::new(glenn, william, 20.0, "glenn_paris_PassWord88");
    ///
    /// let genesis = Block::default(); // that's the actual genesis block
    ///
    /// let new_block = Block::new(1, genesis.hash(), vec![transaction]);
    /// 
    /// assert_eq!(new_block.index(), 1);
    /// ```
    pub fn new(index: usize, prev_hash: [u8; 64], transactions: Vec<Transaction>) -> Self {
        let mut block = Self {
            index,
            prev_hash,
            transactions,
            nonce: 0,
            time: Utc::now(),
            hash: [0; 64],
        };

        block.calculate_hash();

        block
    }

    /// This method returns the hash of the block, since the `hash` field isn't `pub`.
    /// 
    /// # Example
    /// ```
    /// # use blockchain::block::Block;
    /// let genesis_block = Block::default();
    /// 
    /// assert_eq!(genesis_block.hash().len(), 64); // the hash changes everytime because of the time
    /// ```
    pub fn hash(&self) -> [u8; 64] {
        self.hash
    }

    /// This method returns the index of the block, since the `index` field isn't `pub`.
    /// 
    /// # Example
    /// ```
    /// # use blockchain::block::Block;
    /// let genesis_block = Block::default();
    /// 
    /// assert_eq!(genesis_block.index(), 0);
    /// ```
    #[allow(dead_code)]
    pub fn index(&self) -> usize {
        self.index
    }

    /// This method is called when a new block is generated,
    /// and it is used to calculate the SHA-512 hash of the new block.
    /// 
    /// The hash is calculated by using:
    /// - the index of the block
    /// - the previous hash
    /// - the `Transaction`s hashes
    /// - the `DateTime<Utc>` time when the block was generated
    /// - the nonce used for the proof of work
    /// 
    /// The proof of work is checked in the condition of the while loop.
    fn calculate_hash(&mut self) {
        while self.hash[0..2] != [69, 69] {
            let mut hasher = Sha512::new();

            let transactions_hashes = self.transactions.iter().fold(String::new(), |acc, t| format!("{:?}{:?}", acc, t.hash()));
            
            let digest = format!("{}{:?}{}{:?}{}",
                self.index,
                self.prev_hash,
                transactions_hashes,
                self.time,
                self.nonce
            );
    
            hasher.update(digest.as_bytes());
            
            self.hash = hasher
                .finalize()[..]
                .try_into()
                .expect("Error generating the SHA-512 hash of the block.");

            self.nonce += 1;
        }
    }
}

impl Default for Block {
    fn default() -> Self {
        Block::new(0, [0; 64], Vec::new())
    }
}
