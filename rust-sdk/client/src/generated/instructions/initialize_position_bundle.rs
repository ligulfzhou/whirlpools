//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct InitializePositionBundle {
    pub position_bundle: solana_program::pubkey::Pubkey,

    pub position_bundle_mint: solana_program::pubkey::Pubkey,

    pub position_bundle_token_account: solana_program::pubkey::Pubkey,

    pub position_bundle_owner: solana_program::pubkey::Pubkey,

    pub funder: solana_program::pubkey::Pubkey,

    pub token_program: solana_program::pubkey::Pubkey,

    pub system_program: solana_program::pubkey::Pubkey,

    pub rent: solana_program::pubkey::Pubkey,

    pub associated_token_program: solana_program::pubkey::Pubkey,
}

impl InitializePositionBundle {
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(&[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(9 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.position_bundle,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.position_bundle_mint,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.position_bundle_token_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.position_bundle_owner,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.funder,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.rent, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.associated_token_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let data = InitializePositionBundleInstructionData::new()
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
pub struct InitializePositionBundleInstructionData {
    discriminator: [u8; 8],
}

impl InitializePositionBundleInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [117, 45, 241, 149, 24, 18, 194, 65],
        }
    }
}

impl Default for InitializePositionBundleInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

/// Instruction builder for `InitializePositionBundle`.
///
/// ### Accounts:
///
///   0. `[writable]` position_bundle
///   1. `[writable, signer]` position_bundle_mint
///   2. `[writable]` position_bundle_token_account
///   3. `[]` position_bundle_owner
///   4. `[writable, signer]` funder
///   5. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
///   6. `[optional]` system_program (default to `11111111111111111111111111111111`)
///   7. `[optional]` rent (default to `SysvarRent111111111111111111111111111111111`)
///   8. `[]` associated_token_program
#[derive(Clone, Debug, Default)]
pub struct InitializePositionBundleBuilder {
    position_bundle: Option<solana_program::pubkey::Pubkey>,
    position_bundle_mint: Option<solana_program::pubkey::Pubkey>,
    position_bundle_token_account: Option<solana_program::pubkey::Pubkey>,
    position_bundle_owner: Option<solana_program::pubkey::Pubkey>,
    funder: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    rent: Option<solana_program::pubkey::Pubkey>,
    associated_token_program: Option<solana_program::pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl InitializePositionBundleBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    pub fn position_bundle(
        &mut self,
        position_bundle: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.position_bundle = Some(position_bundle);
        self
    }
    #[inline(always)]
    pub fn position_bundle_mint(
        &mut self,
        position_bundle_mint: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.position_bundle_mint = Some(position_bundle_mint);
        self
    }
    #[inline(always)]
    pub fn position_bundle_token_account(
        &mut self,
        position_bundle_token_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.position_bundle_token_account = Some(position_bundle_token_account);
        self
    }
    #[inline(always)]
    pub fn position_bundle_owner(
        &mut self,
        position_bundle_owner: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.position_bundle_owner = Some(position_bundle_owner);
        self
    }
    #[inline(always)]
    pub fn funder(&mut self, funder: solana_program::pubkey::Pubkey) -> &mut Self {
        self.funder = Some(funder);
        self
    }
    /// `[optional account, default to 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA']`
    #[inline(always)]
    pub fn token_program(&mut self, token_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_program = Some(token_program);
        self
    }
    /// `[optional account, default to '11111111111111111111111111111111']`
    #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.system_program = Some(system_program);
        self
    }
    /// `[optional account, default to 'SysvarRent111111111111111111111111111111111']`
    #[inline(always)]
    pub fn rent(&mut self, rent: solana_program::pubkey::Pubkey) -> &mut Self {
        self.rent = Some(rent);
        self
    }
    #[inline(always)]
    pub fn associated_token_program(
        &mut self,
        associated_token_program: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.associated_token_program = Some(associated_token_program);
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
        let accounts = InitializePositionBundle {
            position_bundle: self.position_bundle.expect("position_bundle is not set"),
            position_bundle_mint: self
                .position_bundle_mint
                .expect("position_bundle_mint is not set"),
            position_bundle_token_account: self
                .position_bundle_token_account
                .expect("position_bundle_token_account is not set"),
            position_bundle_owner: self
                .position_bundle_owner
                .expect("position_bundle_owner is not set"),
            funder: self.funder.expect("funder is not set"),
            token_program: self.token_program.unwrap_or(solana_program::pubkey!(
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            )),
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
            rent: self.rent.unwrap_or(solana_program::pubkey!(
                "SysvarRent111111111111111111111111111111111"
            )),
            associated_token_program: self
                .associated_token_program
                .expect("associated_token_program is not set"),
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `initialize_position_bundle` CPI accounts.
pub struct InitializePositionBundleCpiAccounts<'a, 'b> {
    pub position_bundle: &'b solana_program::account_info::AccountInfo<'a>,

    pub position_bundle_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub position_bundle_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub position_bundle_owner: &'b solana_program::account_info::AccountInfo<'a>,

    pub funder: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub rent: &'b solana_program::account_info::AccountInfo<'a>,

    pub associated_token_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `initialize_position_bundle` CPI instruction.
pub struct InitializePositionBundleCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub position_bundle: &'b solana_program::account_info::AccountInfo<'a>,

    pub position_bundle_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub position_bundle_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub position_bundle_owner: &'b solana_program::account_info::AccountInfo<'a>,

    pub funder: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub rent: &'b solana_program::account_info::AccountInfo<'a>,

    pub associated_token_program: &'b solana_program::account_info::AccountInfo<'a>,
}

impl<'a, 'b> InitializePositionBundleCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: InitializePositionBundleCpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            position_bundle: accounts.position_bundle,
            position_bundle_mint: accounts.position_bundle_mint,
            position_bundle_token_account: accounts.position_bundle_token_account,
            position_bundle_owner: accounts.position_bundle_owner,
            funder: accounts.funder,
            token_program: accounts.token_program,
            system_program: accounts.system_program,
            rent: accounts.rent,
            associated_token_program: accounts.associated_token_program,
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
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.position_bundle.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.position_bundle_mint.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.position_bundle_token_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.position_bundle_owner.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.funder.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.rent.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.associated_token_program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let data = InitializePositionBundleInstructionData::new()
            .try_to_vec()
            .unwrap();

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::WHIRLPOOL_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(10 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.position_bundle.clone());
        account_infos.push(self.position_bundle_mint.clone());
        account_infos.push(self.position_bundle_token_account.clone());
        account_infos.push(self.position_bundle_owner.clone());
        account_infos.push(self.funder.clone());
        account_infos.push(self.token_program.clone());
        account_infos.push(self.system_program.clone());
        account_infos.push(self.rent.clone());
        account_infos.push(self.associated_token_program.clone());
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

/// Instruction builder for `InitializePositionBundle` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` position_bundle
///   1. `[writable, signer]` position_bundle_mint
///   2. `[writable]` position_bundle_token_account
///   3. `[]` position_bundle_owner
///   4. `[writable, signer]` funder
///   5. `[]` token_program
///   6. `[]` system_program
///   7. `[]` rent
///   8. `[]` associated_token_program
#[derive(Clone, Debug)]
pub struct InitializePositionBundleCpiBuilder<'a, 'b> {
    instruction: Box<InitializePositionBundleCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> InitializePositionBundleCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(InitializePositionBundleCpiBuilderInstruction {
            __program: program,
            position_bundle: None,
            position_bundle_mint: None,
            position_bundle_token_account: None,
            position_bundle_owner: None,
            funder: None,
            token_program: None,
            system_program: None,
            rent: None,
            associated_token_program: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    #[inline(always)]
    pub fn position_bundle(
        &mut self,
        position_bundle: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.position_bundle = Some(position_bundle);
        self
    }
    #[inline(always)]
    pub fn position_bundle_mint(
        &mut self,
        position_bundle_mint: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.position_bundle_mint = Some(position_bundle_mint);
        self
    }
    #[inline(always)]
    pub fn position_bundle_token_account(
        &mut self,
        position_bundle_token_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.position_bundle_token_account = Some(position_bundle_token_account);
        self
    }
    #[inline(always)]
    pub fn position_bundle_owner(
        &mut self,
        position_bundle_owner: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.position_bundle_owner = Some(position_bundle_owner);
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
    pub fn token_program(
        &mut self,
        token_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_program = Some(token_program);
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
    pub fn rent(&mut self, rent: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.rent = Some(rent);
        self
    }
    #[inline(always)]
    pub fn associated_token_program(
        &mut self,
        associated_token_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.associated_token_program = Some(associated_token_program);
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
        let instruction = InitializePositionBundleCpi {
            __program: self.instruction.__program,

            position_bundle: self
                .instruction
                .position_bundle
                .expect("position_bundle is not set"),

            position_bundle_mint: self
                .instruction
                .position_bundle_mint
                .expect("position_bundle_mint is not set"),

            position_bundle_token_account: self
                .instruction
                .position_bundle_token_account
                .expect("position_bundle_token_account is not set"),

            position_bundle_owner: self
                .instruction
                .position_bundle_owner
                .expect("position_bundle_owner is not set"),

            funder: self.instruction.funder.expect("funder is not set"),

            token_program: self
                .instruction
                .token_program
                .expect("token_program is not set"),

            system_program: self
                .instruction
                .system_program
                .expect("system_program is not set"),

            rent: self.instruction.rent.expect("rent is not set"),

            associated_token_program: self
                .instruction
                .associated_token_program
                .expect("associated_token_program is not set"),
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct InitializePositionBundleCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    position_bundle: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    position_bundle_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    position_bundle_token_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    position_bundle_owner: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    funder: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    rent: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    associated_token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}