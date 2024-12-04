//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct CollectFees {
    pub whirlpool: solana_program::pubkey::Pubkey,

    pub position_authority: solana_program::pubkey::Pubkey,

    pub position: solana_program::pubkey::Pubkey,

    pub position_token_account: solana_program::pubkey::Pubkey,

    pub token_owner_account_a: solana_program::pubkey::Pubkey,

    pub token_vault_a: solana_program::pubkey::Pubkey,

    pub token_owner_account_b: solana_program::pubkey::Pubkey,

    pub token_vault_b: solana_program::pubkey::Pubkey,

    pub token_program: solana_program::pubkey::Pubkey,
}

impl CollectFees {
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(&[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(9 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.whirlpool,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.position_authority,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.position,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.position_token_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.token_owner_account_a,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.token_vault_a,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.token_owner_account_b,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.token_vault_b,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let data = CollectFeesInstructionData::new().try_to_vec().unwrap();

        solana_program::instruction::Instruction {
            program_id: crate::WHIRLPOOL_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct CollectFeesInstructionData {
    discriminator: [u8; 8],
}

impl CollectFeesInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [164, 152, 207, 99, 30, 186, 19, 182],
        }
    }
}

impl Default for CollectFeesInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

/// Instruction builder for `CollectFees`.
///
/// ### Accounts:
///
///   0. `[]` whirlpool
///   1. `[signer]` position_authority
///   2. `[writable]` position
///   3. `[]` position_token_account
///   4. `[writable]` token_owner_account_a
///   5. `[writable]` token_vault_a
///   6. `[writable]` token_owner_account_b
///   7. `[writable]` token_vault_b
///   8. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
#[derive(Clone, Debug, Default)]
pub struct CollectFeesBuilder {
    whirlpool: Option<solana_program::pubkey::Pubkey>,
    position_authority: Option<solana_program::pubkey::Pubkey>,
    position: Option<solana_program::pubkey::Pubkey>,
    position_token_account: Option<solana_program::pubkey::Pubkey>,
    token_owner_account_a: Option<solana_program::pubkey::Pubkey>,
    token_vault_a: Option<solana_program::pubkey::Pubkey>,
    token_owner_account_b: Option<solana_program::pubkey::Pubkey>,
    token_vault_b: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl CollectFeesBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    pub fn whirlpool(&mut self, whirlpool: solana_program::pubkey::Pubkey) -> &mut Self {
        self.whirlpool = Some(whirlpool);
        self
    }
    #[inline(always)]
    pub fn position_authority(
        &mut self,
        position_authority: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.position_authority = Some(position_authority);
        self
    }
    #[inline(always)]
    pub fn position(&mut self, position: solana_program::pubkey::Pubkey) -> &mut Self {
        self.position = Some(position);
        self
    }
    #[inline(always)]
    pub fn position_token_account(
        &mut self,
        position_token_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.position_token_account = Some(position_token_account);
        self
    }
    #[inline(always)]
    pub fn token_owner_account_a(
        &mut self,
        token_owner_account_a: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.token_owner_account_a = Some(token_owner_account_a);
        self
    }
    #[inline(always)]
    pub fn token_vault_a(&mut self, token_vault_a: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_vault_a = Some(token_vault_a);
        self
    }
    #[inline(always)]
    pub fn token_owner_account_b(
        &mut self,
        token_owner_account_b: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.token_owner_account_b = Some(token_owner_account_b);
        self
    }
    #[inline(always)]
    pub fn token_vault_b(&mut self, token_vault_b: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_vault_b = Some(token_vault_b);
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
        let accounts = CollectFees {
            whirlpool: self.whirlpool.expect("whirlpool is not set"),
            position_authority: self
                .position_authority
                .expect("position_authority is not set"),
            position: self.position.expect("position is not set"),
            position_token_account: self
                .position_token_account
                .expect("position_token_account is not set"),
            token_owner_account_a: self
                .token_owner_account_a
                .expect("token_owner_account_a is not set"),
            token_vault_a: self.token_vault_a.expect("token_vault_a is not set"),
            token_owner_account_b: self
                .token_owner_account_b
                .expect("token_owner_account_b is not set"),
            token_vault_b: self.token_vault_b.expect("token_vault_b is not set"),
            token_program: self.token_program.unwrap_or(solana_program::pubkey!(
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            )),
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `collect_fees` CPI accounts.
pub struct CollectFeesCpiAccounts<'a, 'b> {
    pub whirlpool: &'b solana_program::account_info::AccountInfo<'a>,

    pub position_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub position: &'b solana_program::account_info::AccountInfo<'a>,

    pub position_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_owner_account_a: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_vault_a: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_owner_account_b: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_vault_b: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `collect_fees` CPI instruction.
pub struct CollectFeesCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub whirlpool: &'b solana_program::account_info::AccountInfo<'a>,

    pub position_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub position: &'b solana_program::account_info::AccountInfo<'a>,

    pub position_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_owner_account_a: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_vault_a: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_owner_account_b: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_vault_b: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
}

impl<'a, 'b> CollectFeesCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: CollectFeesCpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            whirlpool: accounts.whirlpool,
            position_authority: accounts.position_authority,
            position: accounts.position,
            position_token_account: accounts.position_token_account,
            token_owner_account_a: accounts.token_owner_account_a,
            token_vault_a: accounts.token_vault_a,
            token_owner_account_b: accounts.token_owner_account_b,
            token_vault_b: accounts.token_vault_b,
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
        let mut accounts = Vec::with_capacity(9 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.whirlpool.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.position_authority.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.position.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.position_token_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.token_owner_account_a.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.token_vault_a.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.token_owner_account_b.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.token_vault_b.key,
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
        let data = CollectFeesInstructionData::new().try_to_vec().unwrap();

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::WHIRLPOOL_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(10 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.whirlpool.clone());
        account_infos.push(self.position_authority.clone());
        account_infos.push(self.position.clone());
        account_infos.push(self.position_token_account.clone());
        account_infos.push(self.token_owner_account_a.clone());
        account_infos.push(self.token_vault_a.clone());
        account_infos.push(self.token_owner_account_b.clone());
        account_infos.push(self.token_vault_b.clone());
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

/// Instruction builder for `CollectFees` via CPI.
///
/// ### Accounts:
///
///   0. `[]` whirlpool
///   1. `[signer]` position_authority
///   2. `[writable]` position
///   3. `[]` position_token_account
///   4. `[writable]` token_owner_account_a
///   5. `[writable]` token_vault_a
///   6. `[writable]` token_owner_account_b
///   7. `[writable]` token_vault_b
///   8. `[]` token_program
#[derive(Clone, Debug)]
pub struct CollectFeesCpiBuilder<'a, 'b> {
    instruction: Box<CollectFeesCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> CollectFeesCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(CollectFeesCpiBuilderInstruction {
            __program: program,
            whirlpool: None,
            position_authority: None,
            position: None,
            position_token_account: None,
            token_owner_account_a: None,
            token_vault_a: None,
            token_owner_account_b: None,
            token_vault_b: None,
            token_program: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
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
    pub fn position_authority(
        &mut self,
        position_authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.position_authority = Some(position_authority);
        self
    }
    #[inline(always)]
    pub fn position(
        &mut self,
        position: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.position = Some(position);
        self
    }
    #[inline(always)]
    pub fn position_token_account(
        &mut self,
        position_token_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.position_token_account = Some(position_token_account);
        self
    }
    #[inline(always)]
    pub fn token_owner_account_a(
        &mut self,
        token_owner_account_a: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_owner_account_a = Some(token_owner_account_a);
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
    pub fn token_owner_account_b(
        &mut self,
        token_owner_account_b: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_owner_account_b = Some(token_owner_account_b);
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
        let instruction = CollectFeesCpi {
            __program: self.instruction.__program,

            whirlpool: self.instruction.whirlpool.expect("whirlpool is not set"),

            position_authority: self
                .instruction
                .position_authority
                .expect("position_authority is not set"),

            position: self.instruction.position.expect("position is not set"),

            position_token_account: self
                .instruction
                .position_token_account
                .expect("position_token_account is not set"),

            token_owner_account_a: self
                .instruction
                .token_owner_account_a
                .expect("token_owner_account_a is not set"),

            token_vault_a: self
                .instruction
                .token_vault_a
                .expect("token_vault_a is not set"),

            token_owner_account_b: self
                .instruction
                .token_owner_account_b
                .expect("token_owner_account_b is not set"),

            token_vault_b: self
                .instruction
                .token_vault_b
                .expect("token_vault_b is not set"),

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
struct CollectFeesCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    whirlpool: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    position_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    position: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    position_token_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_owner_account_a: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_vault_a: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_owner_account_b: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_vault_b: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}