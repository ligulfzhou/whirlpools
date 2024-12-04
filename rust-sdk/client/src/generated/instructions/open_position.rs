//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct OpenPosition {
    pub funder: solana_program::pubkey::Pubkey,

    pub owner: solana_program::pubkey::Pubkey,

    pub position: solana_program::pubkey::Pubkey,

    pub position_mint: solana_program::pubkey::Pubkey,

    pub position_token_account: solana_program::pubkey::Pubkey,

    pub whirlpool: solana_program::pubkey::Pubkey,

    pub token_program: solana_program::pubkey::Pubkey,

    pub system_program: solana_program::pubkey::Pubkey,

    pub rent: solana_program::pubkey::Pubkey,

    pub associated_token_program: solana_program::pubkey::Pubkey,
}

impl OpenPosition {
    pub fn instruction(
        &self,
        args: OpenPositionInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: OpenPositionInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(10 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.funder,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.owner, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.position,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.position_mint,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.position_token_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.whirlpool,
            false,
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
        let mut data = OpenPositionInstructionData::new().try_to_vec().unwrap();
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
pub struct OpenPositionInstructionData {
    discriminator: [u8; 8],
}

impl OpenPositionInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [135, 128, 47, 77, 15, 152, 240, 49],
        }
    }
}

impl Default for OpenPositionInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OpenPositionInstructionArgs {
    pub position_bump: u8,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
}

/// Instruction builder for `OpenPosition`.
///
/// ### Accounts:
///
///   0. `[writable, signer]` funder
///   1. `[]` owner
///   2. `[writable]` position
///   3. `[writable, signer]` position_mint
///   4. `[writable]` position_token_account
///   5. `[]` whirlpool
///   6. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
///   7. `[optional]` system_program (default to `11111111111111111111111111111111`)
///   8. `[optional]` rent (default to `SysvarRent111111111111111111111111111111111`)
///   9. `[]` associated_token_program
#[derive(Clone, Debug, Default)]
pub struct OpenPositionBuilder {
    funder: Option<solana_program::pubkey::Pubkey>,
    owner: Option<solana_program::pubkey::Pubkey>,
    position: Option<solana_program::pubkey::Pubkey>,
    position_mint: Option<solana_program::pubkey::Pubkey>,
    position_token_account: Option<solana_program::pubkey::Pubkey>,
    whirlpool: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    rent: Option<solana_program::pubkey::Pubkey>,
    associated_token_program: Option<solana_program::pubkey::Pubkey>,
    position_bump: Option<u8>,
    tick_lower_index: Option<i32>,
    tick_upper_index: Option<i32>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl OpenPositionBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    pub fn funder(&mut self, funder: solana_program::pubkey::Pubkey) -> &mut Self {
        self.funder = Some(funder);
        self
    }
    #[inline(always)]
    pub fn owner(&mut self, owner: solana_program::pubkey::Pubkey) -> &mut Self {
        self.owner = Some(owner);
        self
    }
    #[inline(always)]
    pub fn position(&mut self, position: solana_program::pubkey::Pubkey) -> &mut Self {
        self.position = Some(position);
        self
    }
    #[inline(always)]
    pub fn position_mint(&mut self, position_mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.position_mint = Some(position_mint);
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
    pub fn whirlpool(&mut self, whirlpool: solana_program::pubkey::Pubkey) -> &mut Self {
        self.whirlpool = Some(whirlpool);
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
    #[inline(always)]
    pub fn position_bump(&mut self, position_bump: u8) -> &mut Self {
        self.position_bump = Some(position_bump);
        self
    }
    #[inline(always)]
    pub fn tick_lower_index(&mut self, tick_lower_index: i32) -> &mut Self {
        self.tick_lower_index = Some(tick_lower_index);
        self
    }
    #[inline(always)]
    pub fn tick_upper_index(&mut self, tick_upper_index: i32) -> &mut Self {
        self.tick_upper_index = Some(tick_upper_index);
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
        let accounts = OpenPosition {
            funder: self.funder.expect("funder is not set"),
            owner: self.owner.expect("owner is not set"),
            position: self.position.expect("position is not set"),
            position_mint: self.position_mint.expect("position_mint is not set"),
            position_token_account: self
                .position_token_account
                .expect("position_token_account is not set"),
            whirlpool: self.whirlpool.expect("whirlpool is not set"),
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
        let args = OpenPositionInstructionArgs {
            position_bump: self
                .position_bump
                .clone()
                .expect("position_bump is not set"),
            tick_lower_index: self
                .tick_lower_index
                .clone()
                .expect("tick_lower_index is not set"),
            tick_upper_index: self
                .tick_upper_index
                .clone()
                .expect("tick_upper_index is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `open_position` CPI accounts.
pub struct OpenPositionCpiAccounts<'a, 'b> {
    pub funder: &'b solana_program::account_info::AccountInfo<'a>,

    pub owner: &'b solana_program::account_info::AccountInfo<'a>,

    pub position: &'b solana_program::account_info::AccountInfo<'a>,

    pub position_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub position_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub whirlpool: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub rent: &'b solana_program::account_info::AccountInfo<'a>,

    pub associated_token_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `open_position` CPI instruction.
pub struct OpenPositionCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub funder: &'b solana_program::account_info::AccountInfo<'a>,

    pub owner: &'b solana_program::account_info::AccountInfo<'a>,

    pub position: &'b solana_program::account_info::AccountInfo<'a>,

    pub position_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub position_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub whirlpool: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub rent: &'b solana_program::account_info::AccountInfo<'a>,

    pub associated_token_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: OpenPositionInstructionArgs,
}

impl<'a, 'b> OpenPositionCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: OpenPositionCpiAccounts<'a, 'b>,
        args: OpenPositionInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            funder: accounts.funder,
            owner: accounts.owner,
            position: accounts.position,
            position_mint: accounts.position_mint,
            position_token_account: accounts.position_token_account,
            whirlpool: accounts.whirlpool,
            token_program: accounts.token_program,
            system_program: accounts.system_program,
            rent: accounts.rent,
            associated_token_program: accounts.associated_token_program,
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
        let mut accounts = Vec::with_capacity(10 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.funder.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.owner.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.position.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.position_mint.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.position_token_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.whirlpool.key,
            false,
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
        let mut data = OpenPositionInstructionData::new().try_to_vec().unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::WHIRLPOOL_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(11 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.funder.clone());
        account_infos.push(self.owner.clone());
        account_infos.push(self.position.clone());
        account_infos.push(self.position_mint.clone());
        account_infos.push(self.position_token_account.clone());
        account_infos.push(self.whirlpool.clone());
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

/// Instruction builder for `OpenPosition` via CPI.
///
/// ### Accounts:
///
///   0. `[writable, signer]` funder
///   1. `[]` owner
///   2. `[writable]` position
///   3. `[writable, signer]` position_mint
///   4. `[writable]` position_token_account
///   5. `[]` whirlpool
///   6. `[]` token_program
///   7. `[]` system_program
///   8. `[]` rent
///   9. `[]` associated_token_program
#[derive(Clone, Debug)]
pub struct OpenPositionCpiBuilder<'a, 'b> {
    instruction: Box<OpenPositionCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> OpenPositionCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(OpenPositionCpiBuilderInstruction {
            __program: program,
            funder: None,
            owner: None,
            position: None,
            position_mint: None,
            position_token_account: None,
            whirlpool: None,
            token_program: None,
            system_program: None,
            rent: None,
            associated_token_program: None,
            position_bump: None,
            tick_lower_index: None,
            tick_upper_index: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
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
    pub fn owner(&mut self, owner: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.owner = Some(owner);
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
    pub fn position_mint(
        &mut self,
        position_mint: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.position_mint = Some(position_mint);
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
    pub fn whirlpool(
        &mut self,
        whirlpool: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.whirlpool = Some(whirlpool);
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
    #[inline(always)]
    pub fn position_bump(&mut self, position_bump: u8) -> &mut Self {
        self.instruction.position_bump = Some(position_bump);
        self
    }
    #[inline(always)]
    pub fn tick_lower_index(&mut self, tick_lower_index: i32) -> &mut Self {
        self.instruction.tick_lower_index = Some(tick_lower_index);
        self
    }
    #[inline(always)]
    pub fn tick_upper_index(&mut self, tick_upper_index: i32) -> &mut Self {
        self.instruction.tick_upper_index = Some(tick_upper_index);
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
        let args = OpenPositionInstructionArgs {
            position_bump: self
                .instruction
                .position_bump
                .clone()
                .expect("position_bump is not set"),
            tick_lower_index: self
                .instruction
                .tick_lower_index
                .clone()
                .expect("tick_lower_index is not set"),
            tick_upper_index: self
                .instruction
                .tick_upper_index
                .clone()
                .expect("tick_upper_index is not set"),
        };
        let instruction = OpenPositionCpi {
            __program: self.instruction.__program,

            funder: self.instruction.funder.expect("funder is not set"),

            owner: self.instruction.owner.expect("owner is not set"),

            position: self.instruction.position.expect("position is not set"),

            position_mint: self
                .instruction
                .position_mint
                .expect("position_mint is not set"),

            position_token_account: self
                .instruction
                .position_token_account
                .expect("position_token_account is not set"),

            whirlpool: self.instruction.whirlpool.expect("whirlpool is not set"),

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
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct OpenPositionCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    funder: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    owner: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    position: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    position_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    position_token_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    whirlpool: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    rent: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    associated_token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    position_bump: Option<u8>,
    tick_lower_index: Option<i32>,
    tick_upper_index: Option<i32>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
