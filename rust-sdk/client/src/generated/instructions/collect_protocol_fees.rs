//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct CollectProtocolFees {
    pub whirlpools_config: solana_program::pubkey::Pubkey,

    pub whirlpool: solana_program::pubkey::Pubkey,

    pub collect_protocol_fees_authority: solana_program::pubkey::Pubkey,

    pub token_vault_a: solana_program::pubkey::Pubkey,

    pub token_vault_b: solana_program::pubkey::Pubkey,

    pub token_destination_a: solana_program::pubkey::Pubkey,

    pub token_destination_b: solana_program::pubkey::Pubkey,

    pub token_program: solana_program::pubkey::Pubkey,
}

impl CollectProtocolFees {
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(&[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(8 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.whirlpools_config,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.whirlpool,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.collect_protocol_fees_authority,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.token_vault_a,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.token_vault_b,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.token_destination_a,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.token_destination_b,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let data = CollectProtocolFeesInstructionData::new()
            .try_to_vec()
            .unwrap();

        solana_program::instruction::Instruction {
            program_id: crate::WHIRLPOOL_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct CollectProtocolFeesInstructionData {
    discriminator: [u8; 8],
}

impl CollectProtocolFeesInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [22, 67, 23, 98, 150, 178, 70, 220],
        }
    }
}

impl Default for CollectProtocolFeesInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

/// Instruction builder for `CollectProtocolFees`.
///
/// ### Accounts:
///
///   0. `[]` whirlpools_config
///   1. `[writable]` whirlpool
///   2. `[signer]` collect_protocol_fees_authority
///   3. `[writable]` token_vault_a
///   4. `[writable]` token_vault_b
///   5. `[writable]` token_destination_a
///   6. `[writable]` token_destination_b
///   7. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
#[derive(Clone, Debug, Default)]
pub struct CollectProtocolFeesBuilder {
    whirlpools_config: Option<solana_program::pubkey::Pubkey>,
    whirlpool: Option<solana_program::pubkey::Pubkey>,
    collect_protocol_fees_authority: Option<solana_program::pubkey::Pubkey>,
    token_vault_a: Option<solana_program::pubkey::Pubkey>,
    token_vault_b: Option<solana_program::pubkey::Pubkey>,
    token_destination_a: Option<solana_program::pubkey::Pubkey>,
    token_destination_b: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl CollectProtocolFeesBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    pub fn whirlpools_config(
        &mut self,
        whirlpools_config: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.whirlpools_config = Some(whirlpools_config);
        self
    }
    #[inline(always)]
    pub fn whirlpool(&mut self, whirlpool: solana_program::pubkey::Pubkey) -> &mut Self {
        self.whirlpool = Some(whirlpool);
        self
    }
    #[inline(always)]
    pub fn collect_protocol_fees_authority(
        &mut self,
        collect_protocol_fees_authority: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.collect_protocol_fees_authority = Some(collect_protocol_fees_authority);
        self
    }
    #[inline(always)]
    pub fn token_vault_a(&mut self, token_vault_a: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_vault_a = Some(token_vault_a);
        self
    }
    #[inline(always)]
    pub fn token_vault_b(&mut self, token_vault_b: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_vault_b = Some(token_vault_b);
        self
    }
    #[inline(always)]
    pub fn token_destination_a(
        &mut self,
        token_destination_a: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.token_destination_a = Some(token_destination_a);
        self
    }
    #[inline(always)]
    pub fn token_destination_b(
        &mut self,
        token_destination_b: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.token_destination_b = Some(token_destination_b);
        self
    }
    /// `[optional account, default to 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA']`
    #[inline(always)]
    pub fn token_program(&mut self, token_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_program = Some(token_program);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = CollectProtocolFees {
            whirlpools_config: self
                .whirlpools_config
                .expect("whirlpools_config is not set"),
            whirlpool: self.whirlpool.expect("whirlpool is not set"),
            collect_protocol_fees_authority: self
                .collect_protocol_fees_authority
                .expect("collect_protocol_fees_authority is not set"),
            token_vault_a: self.token_vault_a.expect("token_vault_a is not set"),
            token_vault_b: self.token_vault_b.expect("token_vault_b is not set"),
            token_destination_a: self
                .token_destination_a
                .expect("token_destination_a is not set"),
            token_destination_b: self
                .token_destination_b
                .expect("token_destination_b is not set"),
            token_program: self.token_program.unwrap_or(solana_program::pubkey!(
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            )),
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `collect_protocol_fees` CPI accounts.
pub struct CollectProtocolFeesCpiAccounts<'a, 'b> {
    pub whirlpools_config: &'b solana_program::account_info::AccountInfo<'a>,

    pub whirlpool: &'b solana_program::account_info::AccountInfo<'a>,

    pub collect_protocol_fees_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_vault_a: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_vault_b: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_destination_a: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_destination_b: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `collect_protocol_fees` CPI instruction.
pub struct CollectProtocolFeesCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub whirlpools_config: &'b solana_program::account_info::AccountInfo<'a>,

    pub whirlpool: &'b solana_program::account_info::AccountInfo<'a>,

    pub collect_protocol_fees_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_vault_a: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_vault_b: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_destination_a: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_destination_b: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
}

impl<'a, 'b> CollectProtocolFeesCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: CollectProtocolFeesCpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            whirlpools_config: accounts.whirlpools_config,
            whirlpool: accounts.whirlpool,
            collect_protocol_fees_authority: accounts.collect_protocol_fees_authority,
            token_vault_a: accounts.token_vault_a,
            token_vault_b: accounts.token_vault_b,
            token_destination_a: accounts.token_destination_a,
            token_destination_b: accounts.token_destination_b,
            token_program: accounts.token_program,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(8 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.whirlpools_config.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.whirlpool.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.collect_protocol_fees_authority.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.token_vault_a.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.token_vault_b.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.token_destination_a.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.token_destination_b.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let data = CollectProtocolFeesInstructionData::new()
            .try_to_vec()
            .unwrap();

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::WHIRLPOOL_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(9 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.whirlpools_config.clone());
        account_infos.push(self.whirlpool.clone());
        account_infos.push(self.collect_protocol_fees_authority.clone());
        account_infos.push(self.token_vault_a.clone());
        account_infos.push(self.token_vault_b.clone());
        account_infos.push(self.token_destination_a.clone());
        account_infos.push(self.token_destination_b.clone());
        account_infos.push(self.token_program.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `CollectProtocolFees` via CPI.
///
/// ### Accounts:
///
///   0. `[]` whirlpools_config
///   1. `[writable]` whirlpool
///   2. `[signer]` collect_protocol_fees_authority
///   3. `[writable]` token_vault_a
///   4. `[writable]` token_vault_b
///   5. `[writable]` token_destination_a
///   6. `[writable]` token_destination_b
///   7. `[]` token_program
#[derive(Clone, Debug)]
pub struct CollectProtocolFeesCpiBuilder<'a, 'b> {
    instruction: Box<CollectProtocolFeesCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> CollectProtocolFeesCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(CollectProtocolFeesCpiBuilderInstruction {
            __program: program,
            whirlpools_config: None,
            whirlpool: None,
            collect_protocol_fees_authority: None,
            token_vault_a: None,
            token_vault_b: None,
            token_destination_a: None,
            token_destination_b: None,
            token_program: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    #[inline(always)]
    pub fn whirlpools_config(
        &mut self,
        whirlpools_config: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.whirlpools_config = Some(whirlpools_config);
        self
    }
    #[inline(always)]
    pub fn whirlpool(
        &mut self,
        whirlpool: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.whirlpool = Some(whirlpool);
        self
    }
    #[inline(always)]
    pub fn collect_protocol_fees_authority(
        &mut self,
        collect_protocol_fees_authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.collect_protocol_fees_authority = Some(collect_protocol_fees_authority);
        self
    }
    #[inline(always)]
    pub fn token_vault_a(
        &mut self,
        token_vault_a: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_vault_a = Some(token_vault_a);
        self
    }
    #[inline(always)]
    pub fn token_vault_b(
        &mut self,
        token_vault_b: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_vault_b = Some(token_vault_b);
        self
    }
    #[inline(always)]
    pub fn token_destination_a(
        &mut self,
        token_destination_a: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_destination_a = Some(token_destination_a);
        self
    }
    #[inline(always)]
    pub fn token_destination_b(
        &mut self,
        token_destination_b: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_destination_b = Some(token_destination_b);
        self
    }
    #[inline(always)]
    pub fn token_program(
        &mut self,
        token_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_program = Some(token_program);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
    /// and a `bool` indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let instruction = CollectProtocolFeesCpi {
            __program: self.instruction.__program,

            whirlpools_config: self
                .instruction
                .whirlpools_config
                .expect("whirlpools_config is not set"),

            whirlpool: self.instruction.whirlpool.expect("whirlpool is not set"),

            collect_protocol_fees_authority: self
                .instruction
                .collect_protocol_fees_authority
                .expect("collect_protocol_fees_authority is not set"),

            token_vault_a: self
                .instruction
                .token_vault_a
                .expect("token_vault_a is not set"),

            token_vault_b: self
                .instruction
                .token_vault_b
                .expect("token_vault_b is not set"),

            token_destination_a: self
                .instruction
                .token_destination_a
                .expect("token_destination_a is not set"),

            token_destination_b: self
                .instruction
                .token_destination_b
                .expect("token_destination_b is not set"),

            token_program: self
                .instruction
                .token_program
                .expect("token_program is not set"),
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct CollectProtocolFeesCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    whirlpools_config: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    whirlpool: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    collect_protocol_fees_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_vault_a: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_vault_b: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_destination_a: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_destination_b: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
