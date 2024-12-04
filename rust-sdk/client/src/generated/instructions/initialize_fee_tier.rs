//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct InitializeFeeTier {
    pub config: solana_program::pubkey::Pubkey,

    pub fee_tier: solana_program::pubkey::Pubkey,

    pub funder: solana_program::pubkey::Pubkey,

    pub fee_authority: solana_program::pubkey::Pubkey,

    pub system_program: solana_program::pubkey::Pubkey,
}

impl InitializeFeeTier {
    pub fn instruction(
        &self,
        args: InitializeFeeTierInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: InitializeFeeTierInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(5 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.config,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.fee_tier,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.funder,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.fee_authority,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = InitializeFeeTierInstructionData::new()
            .try_to_vec()
            .unwrap();
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
pub struct InitializeFeeTierInstructionData {
    discriminator: [u8; 8],
}

impl InitializeFeeTierInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [183, 74, 156, 160, 112, 2, 42, 30],
        }
    }
}

impl Default for InitializeFeeTierInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeFeeTierInstructionArgs {
    pub tick_spacing: u16,
    pub default_fee_rate: u16,
}

/// Instruction builder for `InitializeFeeTier`.
///
/// ### Accounts:
///
///   0. `[]` config
///   1. `[writable]` fee_tier
///   2. `[writable, signer]` funder
///   3. `[signer]` fee_authority
///   4. `[optional]` system_program (default to `11111111111111111111111111111111`)
#[derive(Clone, Debug, Default)]
pub struct InitializeFeeTierBuilder {
    config: Option<solana_program::pubkey::Pubkey>,
    fee_tier: Option<solana_program::pubkey::Pubkey>,
    funder: Option<solana_program::pubkey::Pubkey>,
    fee_authority: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    tick_spacing: Option<u16>,
    default_fee_rate: Option<u16>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl InitializeFeeTierBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    pub fn config(&mut self, config: solana_program::pubkey::Pubkey) -> &mut Self {
        self.config = Some(config);
        self
    }
    #[inline(always)]
    pub fn fee_tier(&mut self, fee_tier: solana_program::pubkey::Pubkey) -> &mut Self {
        self.fee_tier = Some(fee_tier);
        self
    }
    #[inline(always)]
    pub fn funder(&mut self, funder: solana_program::pubkey::Pubkey) -> &mut Self {
        self.funder = Some(funder);
        self
    }
    #[inline(always)]
    pub fn fee_authority(&mut self, fee_authority: solana_program::pubkey::Pubkey) -> &mut Self {
        self.fee_authority = Some(fee_authority);
        self
    }
    /// `[optional account, default to '11111111111111111111111111111111']`
    #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.system_program = Some(system_program);
        self
    }
    #[inline(always)]
    pub fn tick_spacing(&mut self, tick_spacing: u16) -> &mut Self {
        self.tick_spacing = Some(tick_spacing);
        self
    }
    #[inline(always)]
    pub fn default_fee_rate(&mut self, default_fee_rate: u16) -> &mut Self {
        self.default_fee_rate = Some(default_fee_rate);
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
        let accounts = InitializeFeeTier {
            config: self.config.expect("config is not set"),
            fee_tier: self.fee_tier.expect("fee_tier is not set"),
            funder: self.funder.expect("funder is not set"),
            fee_authority: self.fee_authority.expect("fee_authority is not set"),
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
        };
        let args = InitializeFeeTierInstructionArgs {
            tick_spacing: self.tick_spacing.clone().expect("tick_spacing is not set"),
            default_fee_rate: self
                .default_fee_rate
                .clone()
                .expect("default_fee_rate is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `initialize_fee_tier` CPI accounts.
pub struct InitializeFeeTierCpiAccounts<'a, 'b> {
    pub config: &'b solana_program::account_info::AccountInfo<'a>,

    pub fee_tier: &'b solana_program::account_info::AccountInfo<'a>,

    pub funder: &'b solana_program::account_info::AccountInfo<'a>,

    pub fee_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `initialize_fee_tier` CPI instruction.
pub struct InitializeFeeTierCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub config: &'b solana_program::account_info::AccountInfo<'a>,

    pub fee_tier: &'b solana_program::account_info::AccountInfo<'a>,

    pub funder: &'b solana_program::account_info::AccountInfo<'a>,

    pub fee_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: InitializeFeeTierInstructionArgs,
}

impl<'a, 'b> InitializeFeeTierCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: InitializeFeeTierCpiAccounts<'a, 'b>,
        args: InitializeFeeTierInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            config: accounts.config,
            fee_tier: accounts.fee_tier,
            funder: accounts.funder,
            fee_authority: accounts.fee_authority,
            system_program: accounts.system_program,
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
        let mut accounts = Vec::with_capacity(5 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.config.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.fee_tier.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.funder.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.fee_authority.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = InitializeFeeTierInstructionData::new()
            .try_to_vec()
            .unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::WHIRLPOOL_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(6 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.config.clone());
        account_infos.push(self.fee_tier.clone());
        account_infos.push(self.funder.clone());
        account_infos.push(self.fee_authority.clone());
        account_infos.push(self.system_program.clone());
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

/// Instruction builder for `InitializeFeeTier` via CPI.
///
/// ### Accounts:
///
///   0. `[]` config
///   1. `[writable]` fee_tier
///   2. `[writable, signer]` funder
///   3. `[signer]` fee_authority
///   4. `[]` system_program
#[derive(Clone, Debug)]
pub struct InitializeFeeTierCpiBuilder<'a, 'b> {
    instruction: Box<InitializeFeeTierCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> InitializeFeeTierCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(InitializeFeeTierCpiBuilderInstruction {
            __program: program,
            config: None,
            fee_tier: None,
            funder: None,
            fee_authority: None,
            system_program: None,
            tick_spacing: None,
            default_fee_rate: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    #[inline(always)]
    pub fn config(
        &mut self,
        config: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.config = Some(config);
        self
    }
    #[inline(always)]
    pub fn fee_tier(
        &mut self,
        fee_tier: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.fee_tier = Some(fee_tier);
        self
    }
    #[inline(always)]
    pub fn funder(
        &mut self,
        funder: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.funder = Some(funder);
        self
    }
    #[inline(always)]
    pub fn fee_authority(
        &mut self,
        fee_authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.fee_authority = Some(fee_authority);
        self
    }
    #[inline(always)]
    pub fn system_program(
        &mut self,
        system_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.system_program = Some(system_program);
        self
    }
    #[inline(always)]
    pub fn tick_spacing(&mut self, tick_spacing: u16) -> &mut Self {
        self.instruction.tick_spacing = Some(tick_spacing);
        self
    }
    #[inline(always)]
    pub fn default_fee_rate(&mut self, default_fee_rate: u16) -> &mut Self {
        self.instruction.default_fee_rate = Some(default_fee_rate);
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
        let args = InitializeFeeTierInstructionArgs {
            tick_spacing: self
                .instruction
                .tick_spacing
                .clone()
                .expect("tick_spacing is not set"),
            default_fee_rate: self
                .instruction
                .default_fee_rate
                .clone()
                .expect("default_fee_rate is not set"),
        };
        let instruction = InitializeFeeTierCpi {
            __program: self.instruction.__program,

            config: self.instruction.config.expect("config is not set"),

            fee_tier: self.instruction.fee_tier.expect("fee_tier is not set"),

            funder: self.instruction.funder.expect("funder is not set"),

            fee_authority: self
                .instruction
                .fee_authority
                .expect("fee_authority is not set"),

            system_program: self
                .instruction
                .system_program
                .expect("system_program is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct InitializeFeeTierCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    config: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    fee_tier: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    funder: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    fee_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    tick_spacing: Option<u16>,
    default_fee_rate: Option<u16>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
