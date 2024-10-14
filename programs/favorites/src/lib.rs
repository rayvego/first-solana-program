use anchor_lang::prelude::*; // this brings in everything that Anchor has to offer!

// a program has a program id aka address which we need to set up
declare_id!("Cge9tQRBsoRMKMty2tF1taxbv8E3QRnBZHXcN2XrTVah"); // we actually don't need to fill this up in Solana Playground, it is done automatically when we deploy the program.

// this is written to every account on the blockchain by the anchor program, it basically specifies the type of account it is
pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8; // when we save things on the blockchain, we'll need 8 bytes + (size of what we're storing)

// we can convert a rust code with a macro into an anchor program

// upon adding this below line, suddenly the regular rust module becomes a full Solana smart contract!
#[program]
// this is a regular rust module
pub mod favorites {
    use super::*; // this brings everything from the root module (or parent module) into scope, which includes the anchor_lang

    // this is the actual instruction handle, the thing that users are going call
    pub fn set_favorites(
        context: Context<SetFavorites>,
        number: u64,
        color: String,
        hobbies: Vec<String>,
    ) -> Result<()> {
        msg!("Greetings from {}", context.program_id); // messages are basically like console.logs and it writes to the solana log file which could actually be seen when someone makes a transaction calling this instruction

        let user_public_key = context.accounts.user.key();

        msg!(
            "User {}'s favorite number is {}, favorite color is {}",
            user_public_key,
            number,
            color
        );
        msg!("User's hobbies are: {:?}", hobbies);

        context.accounts.favorites.set_inner(Favorites {
            number,
            color,
            hobbies,
        });

        Ok(())
    }
}

// this account is what we're gonna write onto the blockchain for every user
#[account]
// this macro lets solana know that the below struct Favorites is an Account!
// to create an account with data, we need to determine its space so we know how much rent to pay, etc.
#[derive(InitSpace)] // this macro automatically computes the amount of space that Favorties will require when its stored on chain
                     // this automatic space calculation doesn't happen by magic, we have to set the max space for individual fields
pub struct Favorites {
    pub number: u64,

    #[max_len(50)]
    // since strings could be of variable length, we need to set it max length, the unit is characters
    pub color: String,

    #[max_len(5, 50)] // means that the vector can at max 5 strings of at max length 50 characters
    pub hobbies: Vec<String>,
}

// when people call our set Favorites function, they're gonna need to specify the accounts they're gonna modify on blockchain
// solana isn't single threaded, it can process multiple things at the same time
// to make this possible

// this struct basically defines the set of accounts required to interact with our program (specifically for set_favorites function defined earlier)

#[derive(Accounts)] // this lets anchor know that
pub struct SetFavorites<'info> {
    // this means the user account can be mutated (changed) during the transaction.
    // It allows you to modify the account's state, like decreasing their balance to pay for transaction fees or making other modifications.
    // In this case, it allows the user's account to pay for rent/fees.
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed, // create the account if it doesn't exist.
        payer = user, // set the payer to user for making the account
        space = ANCHOR_DISCRIMINATOR_SIZE + Favorites::INIT_SPACE, // we'll can use INIT_SPACE to get the size of the favorites struct
        seeds = [b"favorites", user.key().as_ref()], // since this account is a PDA, we'll need some seeds to derive the address for the account
        bump // this is just used to find the correct PDA

        // we need to pass the public key as a reference (as a slice of the PubKey) as the seeds expect a slice and not directly an PubKey object.
        // In Rust, the b prefix before a string literal (like "favorites") means that the string is interpreted as a byte string (&[u8]). 
        // In other words, it converts the string into a byte array of type &[u8] rather than a regular string slice (&str).
        // for example: b"favorites"  is equivalent to &[102, 97, 118, 111, 114, 105, 116, 101, 115]
    )]
    pub favorites: Account<'info, Favorites>,

    // the System Program is needed as it is responsible for basic operations on the blockchain, such as creating new accounts or transferring SOL.
    pub system_program: Program<'info, System>,
}

// In Solana, every transaction must declare the accounts it will read from or write to in advance.
// This is a security and performance feature, ensuring that the transaction doesn't modify unexpected parts of the blockchain.
// The SetFavorites<'info> struct defines which accounts are needed when the set_favorites function is called.

// Something great that is enforced by default is that the person signing the program has to be writing to their own favorites account
// because we have set seeds = [b"favorites", user.key().as_ref()] which includes the signing user's public key

// Txn ID for deploying the program: 3hrTZrjjJN9db3xjGU7H1iLQCwm1WaohtFLDZY1e17bKHgHwpqTnuvc6VLqqxiMd2zCTYHQVBH47M6adEyZSm6G5
// Upgradation ID: AnSaS9vQNkVGDcpmAhwMyaK1VhC1TTik2WvogsqpXKAee68iFMgzFF5nqn9eTeDoGmeovjdGZxQ2ircC6okHN72
// Test Txn ID: 5mkrAYhTzbNVCqxXad2tqbhvN4SwoDNautgv9faZfuThGBUWUNUv1nY66tPBNnFFP8Fy6cdHpHaqusWCtSZYMAgP