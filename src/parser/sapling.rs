use zcash_protocol::value::ZatBalance;

use super::*;

// TODO: use constants from zcash library
const NU5_PARAM_SAPLING_SPENDS: &[u8; 16] = b"ZTxIdSSpendsHash";
const NU5_PARAM_SAPLING_SPENDS_COMPACT: &[u8; 16] = b"ZTxIdSSpendCHash";
const NU5_PARAM_SAPLING_SPENDS_NONCOMPACT: &[u8; 16] = b"ZTxIdSSpendNHash";
const NU5_PARAM_SAPLING_OUTPUTS: &[u8; 16] = b"ZTxIdSOutputHash";
const NU5_PARAM_SAPLING_OUTPUTS_COMPACT: &[u8; 16] = b"ZTxIdSOutC__Hash";
const NU5_PARAM_SAPLING_OUTPUTS_MEMO: &[u8; 16] = b"ZTxIdSOutM__Hash";
const NU5_PARAM_SAPLING_OUTPUTS_NONCOMPACT: &[u8; 16] = b"ZTxIdSOutN__Hash";

impl Parser {
    pub fn parse_sapling(
        &mut self,
        ctx: &mut ParserCtx<'_>,
        reader: &mut ByteReader<'_>,
    ) -> Result<(), ParserError> {
        info!("Process sapling");

        let sapling_balance: ZatBalance = ok!({
            let mut tmp = [0u8; 8];
            ok!(reader.read_exact(&mut tmp));
            ZatBalance::from_i64_le_bytes(tmp)
        });

        info!("Sapling balance: {:?}", sapling_balance);
        self.sapling_balance = sapling_balance.into();

        if self.sapling_spend_count > 0 {
            // Read anchor
            // TODO: maybe move anchor to state field?
            ok!(reader.read_exact(&mut self.sapling_anchor));

            // Init hashers
            ctx.hashers
                .tx_compact_hasher
                .init_with_perso(NU5_PARAM_SAPLING_SPENDS_COMPACT);
            ctx.hashers
                .tx_non_compact_hasher
                .init_with_perso(NU5_PARAM_SAPLING_SPENDS_NONCOMPACT);

            self.state = ParserState::ProcessSaplingSpends;
        } else if self.sapling_output_count > 0 {
            // No spends
            // Get empty sapling spends digest
            let sapling_spend = {
                let mut sapling_spend = [0u8; 32];
                let mut tmp_spend_hasher = Blake2b_256::new();
                tmp_spend_hasher.init_with_perso(NU5_PARAM_SAPLING_SPENDS);
                ok!(tmp_spend_hasher.finalize(&mut sapling_spend));

                sapling_spend
            };

            // Update sapling hasher with empty spends digest
            ok!(ctx.hashers.sapling_hasher.update(&sapling_spend));

            // Init outputs hasher
            ctx.hashers
                .tx_compact_hasher
                .init_with_perso(NU5_PARAM_SAPLING_OUTPUTS_COMPACT);

            self.state = ParserState::ProcessSaplingOutputsCompact;
        } else {
            self.state = ParserState::ProcessExtra;
        }

        Ok(())
    }

    pub fn parse_sapling_spends(
        &mut self,
        ctx: &mut ParserCtx<'_>,
        reader: &mut ByteReader<'_>,
    ) -> Result<(), ParserError> {
        info!(
            "Process sapling spends, remaining: {}",
            self.sapling_spend_count - self.sapling_spend_parsed_count
        );

        // TODO: read library hashing impl and fix comments below

        // update non compact hash with cv
        ok!(ctx.hashers.tx_non_compact_hasher.update(&{
            let mut tmp = [0u8; 32];
            ok!(reader.read_exact(&mut tmp));
            tmp
        }));

        // update non compact hash with anchor
        ok!(ctx
            .hashers
            .tx_non_compact_hasher
            .update(&self.sapling_anchor));

        // update compact hash with nullifier
        ok!(ctx.hashers.tx_compact_hasher.update(
            &{
                let mut tmp = [0u8; 32];
                ok!(reader.read_exact(&mut tmp));
                tmp
            }
            .as_ref(),
        ));

        // update non compact hash with rk
        ok!(ctx.hashers.tx_non_compact_hasher.update(
            &{
                let mut tmp = [0u8; 32];
                ok!(reader.read_exact(&mut tmp));
                tmp
            }
            .as_ref(),
        ));

        self.sapling_spend_parsed_count += 1;

        if self.sapling_spend_count == self.sapling_spend_parsed_count {
            info!("All sapling spends parsed");
            self.state = ParserState::ProcessSaplingSpendsHashing;
        }

        Ok(())
    }

    pub fn parse_sapling_spends_hashing(
        &mut self,
        ctx: &mut ParserCtx<'_>,
        reader: &mut ByteReader<'_>,
    ) -> Result<(), ParserError> {
        info!("Process sapling spends hashing");

        // Finalize compact and noncompact sapling spend hashes
        let mut sapling_spend_compact_digest = [0u8; 32];
        ok!(ctx
            .hashers
            .tx_compact_hasher
            .finalize(&mut sapling_spend_compact_digest));
        debug!(
            "Sapling spend compact digest: {}",
            HexSlice(&sapling_spend_compact_digest)
        );

        let mut sapling_spend_non_compact_digest = [0u8; 32];
        ok!(ctx
            .hashers
            .tx_non_compact_hasher
            .finalize(&mut sapling_spend_non_compact_digest));
        debug!(
            "Sapling spend non compact digest: {}",
            HexSlice(&sapling_spend_non_compact_digest)
        );

        // Initialize the sapling spend digest context
        let mut tmp_spend_hasher = Blake2b_256::new();
        tmp_spend_hasher.init_with_perso(NU5_PARAM_SAPLING_SPENDS);
        ok!(tmp_spend_hasher.update(&sapling_spend_compact_digest,));
        ok!(tmp_spend_hasher.update(&sapling_spend_non_compact_digest,));

        let mut sapling_spend = [0u8; 32];
        ok!(tmp_spend_hasher.finalize(&mut sapling_spend));

        debug!("Sapling spend digest: {}", HexSlice(&sapling_spend));

        // Update sapling full hasher with sapling spend digest
        ok!(ctx.hashers.sapling_hasher.update(&sapling_spend));

        if self.sapling_output_count > 0 {
            // Init outputs hasher
            ctx.hashers
                .tx_compact_hasher
                .init_with_perso(NU5_PARAM_SAPLING_OUTPUTS_COMPACT);

            self.state = ParserState::ProcessSaplingOutputsCompact;
        } else {
            self.state = ParserState::ProcessExtra;
        }

        Ok(())
    }

    pub fn parse_sapling_outputs_compact(
        &mut self,
        ctx: &mut ParserCtx<'_>,
        reader: &mut ByteReader<'_>,
    ) -> Result<(), ParserError> {
        info!(
            "Process sapling outputs compact, remaining: {}",
            self.sapling_output_count - self.sapling_output_parsed_count
        );

        let compact_size = 32 + 32 + 52; // cmu + ephemeral_key + enc_ciphertext[..52]

        if reader.remaining_len() < compact_size {
            return Err(ParserError::from_str(
                "Not enough data for sapling compact output",
            ));
        }

        ok!(ctx
            .hashers
            .tx_compact_hasher
            .update(&reader.remaining_slice()[..compact_size]));
        ok!(reader.advance(compact_size));

        self.sapling_output_parsed_count += 1;

        if self.sapling_output_count == self.sapling_output_parsed_count {
            info!("All sapling compact outputs parsed");
            // Init memo hasher
            ctx.hashers
                .tx_memo_hasher
                .init_with_perso(NU5_PARAM_SAPLING_OUTPUTS_MEMO);

            // memo_size = 512 each APDU will contain quarter of the memo
            self.state = ParserState::ProcessSaplingOutputsMemo {
                size: self.sapling_output_count * 512,
                remaining_size: self.sapling_output_count * 512,
            };
        }

        Ok(())
    }

    pub fn parse_sapling_outputs_memo(
        &mut self,
        ctx: &mut ParserCtx<'_>,
        reader: &mut ByteReader<'_>,
        size: usize,
        remaining_size: usize,
    ) -> Result<(), ParserError> {
        info!(
            "Process sapling outputs memo, remaining size: {}",
            remaining_size
        );

        let to_read = core::cmp::min(remaining_size, reader.remaining_len());
        let memo_data = &reader.remaining_slice()[..to_read];
        ok!(ctx.hashers.tx_memo_hasher.update(memo_data));
        ok!(reader.advance(to_read));
        let new_remaining_size = remaining_size - to_read;

        if new_remaining_size == 0 {
            info!("All sapling memo data parsed");

            // Init outputs non compact hasher
            ctx.hashers
                .tx_non_compact_hasher
                .init_with_perso(NU5_PARAM_SAPLING_OUTPUTS_NONCOMPACT);

            self.sapling_output_parsed_count = 0;
            self.state = ParserState::ProcessSaplingOutputsNonCompact;
        } else {
            self.state = ParserState::ProcessSaplingOutputsMemo {
                size,
                remaining_size: new_remaining_size,
            };
        }

        Ok(())
    }

    pub fn parse_sapling_outputs_non_compact(
        &mut self,
        ctx: &mut ParserCtx<'_>,
        reader: &mut ByteReader<'_>,
    ) -> Result<(), ParserError> {
        info!(
            "Process sapling outputs non compact, remaining: {}",
            self.sapling_output_count - self.sapling_output_parsed_count
        );

        let non_compact_size = 32 + 16 + 80;

        if reader.remaining_len() < non_compact_size {
            return Err(ParserError::from_str(
                "Not enough data for sapling non compact output",
            ));
        }
        ok!(ctx
            .hashers
            .tx_non_compact_hasher
            .update(&reader.remaining_slice()[..non_compact_size]));
        ok!(reader.advance(non_compact_size));

        self.sapling_output_parsed_count += 1;

        if self.sapling_output_count == self.sapling_output_parsed_count {
            info!("All sapling non compact outputs parsed");
            self.state = ParserState::ProcessSaplingOutputHashing;
        }

        Ok(())
    }

    pub fn parse_sapling_output_hashing(
        &mut self,
        ctx: &mut ParserCtx<'_>,
        reader: &mut ByteReader<'_>,
    ) -> Result<(), ParserError> {
        /*
            // Finalize compact and noncompact sapling spend hashes
            uint8_t saplingOutputCompactDigest[DIGEST_SIZE];
            uint8_t saplingOutputMemoDigest[DIGEST_SIZE];
            uint8_t saplingOutputNonCompactDigest[DIGEST_SIZE];

            blake2b_256_final(&btchip_context_D.transactionHashCompact.blake2b, saplingOutputCompactDigest);
            blake2b_256_final(&btchip_context_D.transactionHashMemo.blake2b, saplingOutputMemoDigest);
            blake2b_256_final(&btchip_context_D.transactionHashNonCompact.blake2b, saplingOutputNonCompactDigest);

            // Initialize the sapling output digest context
            uint8_t saplingOutput[DIGEST_SIZE];
            blake2b_256_init(&btchip_context_D.transactionHashFull.blake2b, (uint8_t *) NU5_PARAM_SAPLING_OUTPUTS);
            blake2b_256_update(&btchip_context_D.transactionHashFull.blake2b, saplingOutputCompactDigest, sizeof(saplingOutputCompactDigest));
            blake2b_256_update(&btchip_context_D.transactionHashFull.blake2b, saplingOutputMemoDigest, sizeof(saplingOutputMemoDigest));
            blake2b_256_update(&btchip_context_D.transactionHashFull.blake2b, saplingOutputNonCompactDigest, sizeof(saplingOutputNonCompactDigest));
            blake2b_256_final(&btchip_context_D.transactionHashFull.blake2b, saplingOutput);

            blake2b_256_update(&btchip_context_D.transactionSaplingFull.blake2b, saplingOutput, sizeof(saplingOutput));

            blake2b_256_update(&btchip_context_D.transactionSaplingFull.blake2b, btchip_context_D.saplingBalance, sizeof(btchip_context_D.saplingBalance));

            if (btchip_context_D.orchardActionCount > 0) {
                // init the orchard actions compact hash
                blake2b_256_init(&btchip_context_D.transactionHashCompact.blake2b, (uint8_t *) NU5_PARAM_ORCHARD_ACTIONS_COMPACT);

                btchip_context_D.transactionContext.orchardActionsRemaining = btchip_context_D.orchardActionCount;
                btchip_context_D.transactionContext.transactionState =
                    BTCHIP_TRANSACTION_PROCESS_ORCHARD_COMPACT;
            } else {
                btchip_context_D.transactionContext.transactionState =
                    BTCHIP_TRANSACTION_PROCESS_EXTRA;
            }

        */
        info!("Finalize sapling outputs hashing");

        // Finalize compact, memo and noncompact sapling output hashes
        let mut sapling_output_compact_digest = [0u8; 32];
        ok!(ctx
            .hashers
            .tx_compact_hasher
            .finalize(&mut sapling_output_compact_digest));
        debug!(
            "Sapling output compact digest: {}",
            HexSlice(&sapling_output_compact_digest)
        );

        let mut sapling_output_memo_digest = [0u8; 32];
        ok!(ctx
            .hashers
            .tx_memo_hasher
            .finalize(&mut sapling_output_memo_digest));
        debug!(
            "Sapling output memo digest: {}",
            HexSlice(&sapling_output_memo_digest)
        );

        let mut sapling_output_non_compact_digest = [0u8; 32];
        ok!(ctx
            .hashers
            .tx_non_compact_hasher
            .finalize(&mut sapling_output_non_compact_digest));
        debug!(
            "Sapling output non compact digest: {}",
            HexSlice(&sapling_output_non_compact_digest)
        );

        // Initialize the sapling output digest context
        let mut tmp_output_hasher = Blake2b_256::new();
        tmp_output_hasher.init_with_perso(NU5_PARAM_SAPLING_OUTPUTS);

        ok!(tmp_output_hasher.update(&sapling_output_compact_digest));
        ok!(tmp_output_hasher.update(&sapling_output_memo_digest));
        ok!(tmp_output_hasher.update(&sapling_output_non_compact_digest));

        let mut sapling_output = [0u8; 32];
        ok!(tmp_output_hasher.finalize(&mut sapling_output));
        debug!("Sapling output digest: {}", HexSlice(&sapling_output));

        // Update sapling full hasher with sapling output digest
        ok!(ctx.hashers.sapling_hasher.update(&sapling_output));
        // Update sapling full hasher with sapling balance
        ok!(ctx
            .hashers
            .sapling_hasher
            .update(&self.sapling_balance.to_le_bytes()));

        if self.orchard_action_count > 0 {
            // init the orchard actions compact hash

            todo!()
        } else {
            self.state = ParserState::ProcessExtra;
        }

        Ok(())
    }
}
