//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct Swap {
    pub token_program: solana_program::pubkey::Pubkey,

    pub token_authority: solana_program::pubkey::Pubkey,

    pub whirlpool: solana_program::pubkey::Pubkey,

    pub token_owner_account_a: solana_program::pubkey::Pubkey,

    pub token_vault_a: solana_program::pubkey::Pubkey,

    pub token_owner_account_b: solana_program::pubkey::Pubkey,

    pub token_vault_b: solana_program::pubkey::Pubkey,

    pub tick_array0: solana_program::pubkey::Pubkey,

    pub tick_array1: solana_program::pubkey::Pubkey,

    pub tick_array2: solana_program::pubkey::Pubkey,

    pub oracle: solana_program::pubkey::Pubkey,
}

impl Swap {
    pub fn instruction(
        &self,
        args: SwapInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: SwapInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(11 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_authority,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.whirlpool,
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
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.tick_array0,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.tick_array1,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.tick_array2,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.oracle,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = SwapInstructionData::new().try_to_vec().unwrap();
        let mut args = args.try_to_vec().unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::WHIRLPOOL_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct SwapInstructionData {
    discriminator: [u8; 8],
}

impl SwapInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [248, 198, 158, 145, 225, 117, 135, 200],
        }
    }
}

impl Default for SwapInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SwapInstructionArgs {
    pub amount: u64,
    pub other_amount_threshold: u64,
    pub sqrt_price_limit: u128,
    pub amount_specified_is_input: bool,
    pub a_to_b: bool,
}

/// Instruction builder for `Swap`.
///
/// ### Accounts:
///
///   0. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
///   1. `[signer]` token_authority
///   2. `[writable]` whirlpool
///   3. `[writable]` token_owner_account_a
///   4. `[writable]` token_vault_a
///   5. `[writable]` token_owner_account_b
///   6. `[writable]` token_vault_b
///   7. `[writable]` tick_array0
///   8. `[writable]` tick_array1
///   9. `[writable]` tick_array2
///   10. `[]` oracle
#[derive(Clone, Debug, Default)]
pub struct SwapBuilder {
    token_program: Option<solana_program::pubkey::Pubkey>,
    token_authority: Option<solana_program::pubkey::Pubkey>,
    whirlpool: Option<solana_program::pubkey::Pubkey>,
    token_owner_account_a: Option<solana_program::pubkey::Pubkey>,
    token_vault_a: Option<solana_program::pubkey::Pubkey>,
    token_owner_account_b: Option<solana_program::pubkey::Pubkey>,
    token_vault_b: Option<solana_program::pubkey::Pubkey>,
    tick_array0: Option<solana_program::pubkey::Pubkey>,
    tick_array1: Option<solana_program::pubkey::Pubkey>,
    tick_array2: Option<solana_program::pubkey::Pubkey>,
    oracle: Option<solana_program::pubkey::Pubkey>,
    amount: Option<u64>,
    other_amount_threshold: Option<u64>,
    sqrt_price_limit: Option<u128>,
    amount_specified_is_input: Option<bool>,
    a_to_b: Option<bool>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl SwapBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// `[optional account, default to 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA']`
    #[inline(always)]
    pub fn token_program(&mut self, token_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_program = Some(token_program);
        self
    }
    #[inline(always)]
    pub fn token_authority(
        &mut self,
        token_authority: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.token_authority = Some(token_authority);
        self
    }
    #[inline(always)]
    pub fn whirlpool(&mut self, whirlpool: solana_program::pubkey::Pubkey) -> &mut Self {
        self.whirlpool = Some(whirlpool);
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
    #[inline(always)]
    pub fn tick_array0(&mut self, tick_array0: solana_program::pubkey::Pubkey) -> &mut Self {
        self.tick_array0 = Some(tick_array0);
        self
    }
    #[inline(always)]
    pub fn tick_array1(&mut self, tick_array1: solana_program::pubkey::Pubkey) -> &mut Self {
        self.tick_array1 = Some(tick_array1);
        self
    }
    #[inline(always)]
    pub fn tick_array2(&mut self, tick_array2: solana_program::pubkey::Pubkey) -> &mut Self {
        self.tick_array2 = Some(tick_array2);
        self
    }
    #[inline(always)]
    pub fn oracle(&mut self, oracle: solana_program::pubkey::Pubkey) -> &mut Self {
        self.oracle = Some(oracle);
        self
    }
    #[inline(always)]
    pub fn amount(&mut self, amount: u64) -> &mut Self {
        self.amount = Some(amount);
        self
    }
    #[inline(always)]
    pub fn other_amount_threshold(&mut self, other_amount_threshold: u64) -> &mut Self {
        self.other_amount_threshold = Some(other_amount_threshold);
        self
    }
    #[inline(always)]
    pub fn sqrt_price_limit(&mut self, sqrt_price_limit: u128) -> &mut Self {
        self.sqrt_price_limit = Some(sqrt_price_limit);
        self
    }
    #[inline(always)]
    pub fn amount_specified_is_input(&mut self, amount_specified_is_input: bool) -> &mut Self {
        self.amount_specified_is_input = Some(amount_specified_is_input);
        self
    }
    #[inline(always)]
    pub fn a_to_b(&mut self, a_to_b: bool) -> &mut Self {
        self.a_to_b = Some(a_to_b);
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
        let accounts = Swap {
            token_program: self.token_program.unwrap_or(solana_program::pubkey!(
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            )),
            token_authority: self.token_authority.expect("token_authority is not set"),
            whirlpool: self.whirlpool.expect("whirlpool is not set"),
            token_owner_account_a: self
                .token_owner_account_a
                .expect("token_owner_account_a is not set"),
            token_vault_a: self.token_vault_a.expect("token_vault_a is not set"),
            token_owner_account_b: self
                .token_owner_account_b
                .expect("token_owner_account_b is not set"),
            token_vault_b: self.token_vault_b.expect("token_vault_b is not set"),
            tick_array0: self.tick_array0.expect("tick_array0 is not set"),
            tick_array1: self.tick_array1.expect("tick_array1 is not set"),
            tick_array2: self.tick_array2.expect("tick_array2 is not set"),
            oracle: self.oracle.expect("oracle is not set"),
        };
        let args = SwapInstructionArgs {
            amount: self.amount.clone().expect("amount is not set"),
            other_amount_threshold: self
                .other_amount_threshold
                .clone()
                .expect("other_amount_threshold is not set"),
            sqrt_price_limit: self
                .sqrt_price_limit
                .clone()
                .expect("sqrt_price_limit is not set"),
            amount_specified_is_input: self
                .amount_specified_is_input
                .clone()
                .expect("amount_specified_is_input is not set"),
            a_to_b: self.a_to_b.clone().expect("a_to_b is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `swap` CPI accounts.
pub struct SwapCpiAccounts<'a, 'b> {
    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub whirlpool: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_owner_account_a: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_vault_a: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_owner_account_b: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_vault_b: &'b solana_program::account_info::AccountInfo<'a>,

    pub tick_array0: &'b solana_program::account_info::AccountInfo<'a>,

    pub tick_array1: &'b solana_program::account_info::AccountInfo<'a>,

    pub tick_array2: &'b solana_program::account_info::AccountInfo<'a>,

    pub oracle: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `swap` CPI instruction.
pub struct SwapCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub whirlpool: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_owner_account_a: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_vault_a: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_owner_account_b: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_vault_b: &'b solana_program::account_info::AccountInfo<'a>,

    pub tick_array0: &'b solana_program::account_info::AccountInfo<'a>,

    pub tick_array1: &'b solana_program::account_info::AccountInfo<'a>,

    pub tick_array2: &'b solana_program::account_info::AccountInfo<'a>,

    pub oracle: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: SwapInstructionArgs,
}

impl<'a, 'b> SwapCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: SwapCpiAccounts<'a, 'b>,
        args: SwapInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            token_program: accounts.token_program,
            token_authority: accounts.token_authority,
            whirlpool: accounts.whirlpool,
            token_owner_account_a: accounts.token_owner_account_a,
            token_vault_a: accounts.token_vault_a,
            token_owner_account_b: accounts.token_owner_account_b,
            token_vault_b: accounts.token_vault_b,
            tick_array0: accounts.tick_array0,
            tick_array1: accounts.tick_array1,
            tick_array2: accounts.tick_array2,
            oracle: accounts.oracle,
            __args: args,
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
        let mut accounts = Vec::with_capacity(11 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_authority.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.whirlpool.key,
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
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.tick_array0.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.tick_array1.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.tick_array2.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.oracle.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = SwapInstructionData::new().try_to_vec().unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::WHIRLPOOL_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(12 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.token_program.clone());
        account_infos.push(self.token_authority.clone());
        account_infos.push(self.whirlpool.clone());
        account_infos.push(self.token_owner_account_a.clone());
        account_infos.push(self.token_vault_a.clone());
        account_infos.push(self.token_owner_account_b.clone());
        account_infos.push(self.token_vault_b.clone());
        account_infos.push(self.tick_array0.clone());
        account_infos.push(self.tick_array1.clone());
        account_infos.push(self.tick_array2.clone());
        account_infos.push(self.oracle.clone());
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

/// Instruction builder for `Swap` via CPI.
///
/// ### Accounts:
///
///   0. `[]` token_program
///   1. `[signer]` token_authority
///   2. `[writable]` whirlpool
///   3. `[writable]` token_owner_account_a
///   4. `[writable]` token_vault_a
///   5. `[writable]` token_owner_account_b
///   6. `[writable]` token_vault_b
///   7. `[writable]` tick_array0
///   8. `[writable]` tick_array1
///   9. `[writable]` tick_array2
///   10. `[]` oracle
#[derive(Clone, Debug)]
pub struct SwapCpiBuilder<'a, 'b> {
    instruction: Box<SwapCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> SwapCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(SwapCpiBuilderInstruction {
            __program: program,
            token_program: None,
            token_authority: None,
            whirlpool: None,
            token_owner_account_a: None,
            token_vault_a: None,
            token_owner_account_b: None,
            token_vault_b: None,
            tick_array0: None,
            tick_array1: None,
            tick_array2: None,
            oracle: None,
            amount: None,
            other_amount_threshold: None,
            sqrt_price_limit: None,
            amount_specified_is_input: None,
            a_to_b: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    #[inline(always)]
    pub fn token_program(
        &mut self,
        token_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_program = Some(token_program);
        self
    }
    #[inline(always)]
    pub fn token_authority(
        &mut self,
        token_authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_authority = Some(token_authority);
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
    pub fn tick_array0(
        &mut self,
        tick_array0: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.tick_array0 = Some(tick_array0);
        self
    }
    #[inline(always)]
    pub fn tick_array1(
        &mut self,
        tick_array1: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.tick_array1 = Some(tick_array1);
        self
    }
    #[inline(always)]
    pub fn tick_array2(
        &mut self,
        tick_array2: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.tick_array2 = Some(tick_array2);
        self
    }
    #[inline(always)]
    pub fn oracle(
        &mut self,
        oracle: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.oracle = Some(oracle);
        self
    }
    #[inline(always)]
    pub fn amount(&mut self, amount: u64) -> &mut Self {
        self.instruction.amount = Some(amount);
        self
    }
    #[inline(always)]
    pub fn other_amount_threshold(&mut self, other_amount_threshold: u64) -> &mut Self {
        self.instruction.other_amount_threshold = Some(other_amount_threshold);
        self
    }
    #[inline(always)]
    pub fn sqrt_price_limit(&mut self, sqrt_price_limit: u128) -> &mut Self {
        self.instruction.sqrt_price_limit = Some(sqrt_price_limit);
        self
    }
    #[inline(always)]
    pub fn amount_specified_is_input(&mut self, amount_specified_is_input: bool) -> &mut Self {
        self.instruction.amount_specified_is_input = Some(amount_specified_is_input);
        self
    }
    #[inline(always)]
    pub fn a_to_b(&mut self, a_to_b: bool) -> &mut Self {
        self.instruction.a_to_b = Some(a_to_b);
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
        let args = SwapInstructionArgs {
            amount: self.instruction.amount.clone().expect("amount is not set"),
            other_amount_threshold: self
                .instruction
                .other_amount_threshold
                .clone()
                .expect("other_amount_threshold is not set"),
            sqrt_price_limit: self
                .instruction
                .sqrt_price_limit
                .clone()
                .expect("sqrt_price_limit is not set"),
            amount_specified_is_input: self
                .instruction
                .amount_specified_is_input
                .clone()
                .expect("amount_specified_is_input is not set"),
            a_to_b: self.instruction.a_to_b.clone().expect("a_to_b is not set"),
        };
        let instruction = SwapCpi {
            __program: self.instruction.__program,

            token_program: self
                .instruction
                .token_program
                .expect("token_program is not set"),

            token_authority: self
                .instruction
                .token_authority
                .expect("token_authority is not set"),

            whirlpool: self.instruction.whirlpool.expect("whirlpool is not set"),

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

            tick_array0: self
                .instruction
                .tick_array0
                .expect("tick_array0 is not set"),

            tick_array1: self
                .instruction
                .tick_array1
                .expect("tick_array1 is not set"),

            tick_array2: self
                .instruction
                .tick_array2
                .expect("tick_array2 is not set"),

            oracle: self.instruction.oracle.expect("oracle is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct SwapCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    whirlpool: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_owner_account_a: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_vault_a: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_owner_account_b: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_vault_b: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    tick_array0: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    tick_array1: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    tick_array2: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    oracle: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    amount: Option<u64>,
    other_amount_threshold: Option<u64>,
    sqrt_price_limit: Option<u128>,
    amount_specified_is_input: Option<bool>,
    a_to_b: Option<bool>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
