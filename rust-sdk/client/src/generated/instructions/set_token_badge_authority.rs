//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct SetTokenBadgeAuthority {
    pub whirlpools_config: solana_program::pubkey::Pubkey,

    pub whirlpools_config_extension: solana_program::pubkey::Pubkey,

    pub config_extension_authority: solana_program::pubkey::Pubkey,

    pub new_token_badge_authority: solana_program::pubkey::Pubkey,
}

impl SetTokenBadgeAuthority {
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(&[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(4 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.whirlpools_config,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.whirlpools_config_extension,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.config_extension_authority,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.new_token_badge_authority,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let data = SetTokenBadgeAuthorityInstructionData::new()
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
pub struct SetTokenBadgeAuthorityInstructionData {
    discriminator: [u8; 8],
}

impl SetTokenBadgeAuthorityInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [207, 202, 4, 32, 205, 79, 13, 178],
        }
    }
}

impl Default for SetTokenBadgeAuthorityInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

/// Instruction builder for `SetTokenBadgeAuthority`.
///
/// ### Accounts:
///
///   0. `[]` whirlpools_config
///   1. `[writable]` whirlpools_config_extension
///   2. `[signer]` config_extension_authority
///   3. `[]` new_token_badge_authority
#[derive(Clone, Debug, Default)]
pub struct SetTokenBadgeAuthorityBuilder {
    whirlpools_config: Option<solana_program::pubkey::Pubkey>,
    whirlpools_config_extension: Option<solana_program::pubkey::Pubkey>,
    config_extension_authority: Option<solana_program::pubkey::Pubkey>,
    new_token_badge_authority: Option<solana_program::pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl SetTokenBadgeAuthorityBuilder {
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
    pub fn whirlpools_config_extension(
        &mut self,
        whirlpools_config_extension: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.whirlpools_config_extension = Some(whirlpools_config_extension);
        self
    }
    #[inline(always)]
    pub fn config_extension_authority(
        &mut self,
        config_extension_authority: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.config_extension_authority = Some(config_extension_authority);
        self
    }
    #[inline(always)]
    pub fn new_token_badge_authority(
        &mut self,
        new_token_badge_authority: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.new_token_badge_authority = Some(new_token_badge_authority);
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
        let accounts = SetTokenBadgeAuthority {
            whirlpools_config: self
                .whirlpools_config
                .expect("whirlpools_config is not set"),
            whirlpools_config_extension: self
                .whirlpools_config_extension
                .expect("whirlpools_config_extension is not set"),
            config_extension_authority: self
                .config_extension_authority
                .expect("config_extension_authority is not set"),
            new_token_badge_authority: self
                .new_token_badge_authority
                .expect("new_token_badge_authority is not set"),
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `set_token_badge_authority` CPI accounts.
pub struct SetTokenBadgeAuthorityCpiAccounts<'a, 'b> {
    pub whirlpools_config: &'b solana_program::account_info::AccountInfo<'a>,

    pub whirlpools_config_extension: &'b solana_program::account_info::AccountInfo<'a>,

    pub config_extension_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub new_token_badge_authority: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `set_token_badge_authority` CPI instruction.
pub struct SetTokenBadgeAuthorityCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub whirlpools_config: &'b solana_program::account_info::AccountInfo<'a>,

    pub whirlpools_config_extension: &'b solana_program::account_info::AccountInfo<'a>,

    pub config_extension_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub new_token_badge_authority: &'b solana_program::account_info::AccountInfo<'a>,
}

impl<'a, 'b> SetTokenBadgeAuthorityCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: SetTokenBadgeAuthorityCpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            whirlpools_config: accounts.whirlpools_config,
            whirlpools_config_extension: accounts.whirlpools_config_extension,
            config_extension_authority: accounts.config_extension_authority,
            new_token_badge_authority: accounts.new_token_badge_authority,
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
        let mut accounts = Vec::with_capacity(4 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.whirlpools_config.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.whirlpools_config_extension.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.config_extension_authority.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.new_token_badge_authority.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let data = SetTokenBadgeAuthorityInstructionData::new()
            .try_to_vec()
            .unwrap();

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::WHIRLPOOL_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(5 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.whirlpools_config.clone());
        account_infos.push(self.whirlpools_config_extension.clone());
        account_infos.push(self.config_extension_authority.clone());
        account_infos.push(self.new_token_badge_authority.clone());
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

/// Instruction builder for `SetTokenBadgeAuthority` via CPI.
///
/// ### Accounts:
///
///   0. `[]` whirlpools_config
///   1. `[writable]` whirlpools_config_extension
///   2. `[signer]` config_extension_authority
///   3. `[]` new_token_badge_authority
#[derive(Clone, Debug)]
pub struct SetTokenBadgeAuthorityCpiBuilder<'a, 'b> {
    instruction: Box<SetTokenBadgeAuthorityCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> SetTokenBadgeAuthorityCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(SetTokenBadgeAuthorityCpiBuilderInstruction {
            __program: program,
            whirlpools_config: None,
            whirlpools_config_extension: None,
            config_extension_authority: None,
            new_token_badge_authority: None,
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
    pub fn whirlpools_config_extension(
        &mut self,
        whirlpools_config_extension: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.whirlpools_config_extension = Some(whirlpools_config_extension);
        self
    }
    #[inline(always)]
    pub fn config_extension_authority(
        &mut self,
        config_extension_authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.config_extension_authority = Some(config_extension_authority);
        self
    }
    #[inline(always)]
    pub fn new_token_badge_authority(
        &mut self,
        new_token_badge_authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.new_token_badge_authority = Some(new_token_badge_authority);
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
        let instruction = SetTokenBadgeAuthorityCpi {
            __program: self.instruction.__program,

            whirlpools_config: self
                .instruction
                .whirlpools_config
                .expect("whirlpools_config is not set"),

            whirlpools_config_extension: self
                .instruction
                .whirlpools_config_extension
                .expect("whirlpools_config_extension is not set"),

            config_extension_authority: self
                .instruction
                .config_extension_authority
                .expect("config_extension_authority is not set"),

            new_token_badge_authority: self
                .instruction
                .new_token_badge_authority
                .expect("new_token_badge_authority is not set"),
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct SetTokenBadgeAuthorityCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    whirlpools_config: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    whirlpools_config_extension: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    config_extension_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    new_token_badge_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}