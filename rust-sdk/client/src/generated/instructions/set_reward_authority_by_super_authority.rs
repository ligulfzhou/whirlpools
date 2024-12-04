//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct SetRewardAuthorityBySuperAuthority {
    pub whirlpools_config: solana_program::pubkey::Pubkey,

    pub whirlpool: solana_program::pubkey::Pubkey,

    pub reward_emissions_super_authority: solana_program::pubkey::Pubkey,

    pub new_reward_authority: solana_program::pubkey::Pubkey,
}

impl SetRewardAuthorityBySuperAuthority {
    pub fn instruction(
        &self,
        args: SetRewardAuthorityBySuperAuthorityInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: SetRewardAuthorityBySuperAuthorityInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(4 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.whirlpools_config,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.whirlpool,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.reward_emissions_super_authority,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.new_reward_authority,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = SetRewardAuthorityBySuperAuthorityInstructionData::new()
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
pub struct SetRewardAuthorityBySuperAuthorityInstructionData {
    discriminator: [u8; 8],
}

impl SetRewardAuthorityBySuperAuthorityInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [240, 154, 201, 198, 148, 93, 56, 25],
        }
    }
}

impl Default for SetRewardAuthorityBySuperAuthorityInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetRewardAuthorityBySuperAuthorityInstructionArgs {
    pub reward_index: u8,
}

/// Instruction builder for `SetRewardAuthorityBySuperAuthority`.
///
/// ### Accounts:
///
///   0. `[]` whirlpools_config
///   1. `[writable]` whirlpool
///   2. `[signer]` reward_emissions_super_authority
///   3. `[]` new_reward_authority
#[derive(Clone, Debug, Default)]
pub struct SetRewardAuthorityBySuperAuthorityBuilder {
    whirlpools_config: Option<solana_program::pubkey::Pubkey>,
    whirlpool: Option<solana_program::pubkey::Pubkey>,
    reward_emissions_super_authority: Option<solana_program::pubkey::Pubkey>,
    new_reward_authority: Option<solana_program::pubkey::Pubkey>,
    reward_index: Option<u8>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl SetRewardAuthorityBySuperAuthorityBuilder {
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
    pub fn reward_emissions_super_authority(
        &mut self,
        reward_emissions_super_authority: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.reward_emissions_super_authority = Some(reward_emissions_super_authority);
        self
    }
    #[inline(always)]
    pub fn new_reward_authority(
        &mut self,
        new_reward_authority: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.new_reward_authority = Some(new_reward_authority);
        self
    }
    #[inline(always)]
    pub fn reward_index(&mut self, reward_index: u8) -> &mut Self {
        self.reward_index = Some(reward_index);
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
        let accounts = SetRewardAuthorityBySuperAuthority {
            whirlpools_config: self
                .whirlpools_config
                .expect("whirlpools_config is not set"),
            whirlpool: self.whirlpool.expect("whirlpool is not set"),
            reward_emissions_super_authority: self
                .reward_emissions_super_authority
                .expect("reward_emissions_super_authority is not set"),
            new_reward_authority: self
                .new_reward_authority
                .expect("new_reward_authority is not set"),
        };
        let args = SetRewardAuthorityBySuperAuthorityInstructionArgs {
            reward_index: self.reward_index.clone().expect("reward_index is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `set_reward_authority_by_super_authority` CPI accounts.
pub struct SetRewardAuthorityBySuperAuthorityCpiAccounts<'a, 'b> {
    pub whirlpools_config: &'b solana_program::account_info::AccountInfo<'a>,

    pub whirlpool: &'b solana_program::account_info::AccountInfo<'a>,

    pub reward_emissions_super_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub new_reward_authority: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `set_reward_authority_by_super_authority` CPI instruction.
pub struct SetRewardAuthorityBySuperAuthorityCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub whirlpools_config: &'b solana_program::account_info::AccountInfo<'a>,

    pub whirlpool: &'b solana_program::account_info::AccountInfo<'a>,

    pub reward_emissions_super_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub new_reward_authority: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: SetRewardAuthorityBySuperAuthorityInstructionArgs,
}

impl<'a, 'b> SetRewardAuthorityBySuperAuthorityCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: SetRewardAuthorityBySuperAuthorityCpiAccounts<'a, 'b>,
        args: SetRewardAuthorityBySuperAuthorityInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            whirlpools_config: accounts.whirlpools_config,
            whirlpool: accounts.whirlpool,
            reward_emissions_super_authority: accounts.reward_emissions_super_authority,
            new_reward_authority: accounts.new_reward_authority,
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
        let mut accounts = Vec::with_capacity(4 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.whirlpools_config.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.whirlpool.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.reward_emissions_super_authority.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.new_reward_authority.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = SetRewardAuthorityBySuperAuthorityInstructionData::new()
            .try_to_vec()
            .unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::WHIRLPOOL_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(5 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.whirlpools_config.clone());
        account_infos.push(self.whirlpool.clone());
        account_infos.push(self.reward_emissions_super_authority.clone());
        account_infos.push(self.new_reward_authority.clone());
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

/// Instruction builder for `SetRewardAuthorityBySuperAuthority` via CPI.
///
/// ### Accounts:
///
///   0. `[]` whirlpools_config
///   1. `[writable]` whirlpool
///   2. `[signer]` reward_emissions_super_authority
///   3. `[]` new_reward_authority
#[derive(Clone, Debug)]
pub struct SetRewardAuthorityBySuperAuthorityCpiBuilder<'a, 'b> {
    instruction: Box<SetRewardAuthorityBySuperAuthorityCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> SetRewardAuthorityBySuperAuthorityCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(SetRewardAuthorityBySuperAuthorityCpiBuilderInstruction {
            __program: program,
            whirlpools_config: None,
            whirlpool: None,
            reward_emissions_super_authority: None,
            new_reward_authority: None,
            reward_index: None,
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
    pub fn reward_emissions_super_authority(
        &mut self,
        reward_emissions_super_authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.reward_emissions_super_authority = Some(reward_emissions_super_authority);
        self
    }
    #[inline(always)]
    pub fn new_reward_authority(
        &mut self,
        new_reward_authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.new_reward_authority = Some(new_reward_authority);
        self
    }
    #[inline(always)]
    pub fn reward_index(&mut self, reward_index: u8) -> &mut Self {
        self.instruction.reward_index = Some(reward_index);
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
        let args = SetRewardAuthorityBySuperAuthorityInstructionArgs {
            reward_index: self
                .instruction
                .reward_index
                .clone()
                .expect("reward_index is not set"),
        };
        let instruction = SetRewardAuthorityBySuperAuthorityCpi {
            __program: self.instruction.__program,

            whirlpools_config: self
                .instruction
                .whirlpools_config
                .expect("whirlpools_config is not set"),

            whirlpool: self.instruction.whirlpool.expect("whirlpool is not set"),

            reward_emissions_super_authority: self
                .instruction
                .reward_emissions_super_authority
                .expect("reward_emissions_super_authority is not set"),

            new_reward_authority: self
                .instruction
                .new_reward_authority
                .expect("new_reward_authority is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct SetRewardAuthorityBySuperAuthorityCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    whirlpools_config: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    whirlpool: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    reward_emissions_super_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    new_reward_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    reward_index: Option<u8>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
