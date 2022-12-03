use anchor_lang::prelude::*;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("11111111111111111111111111111111");

// Artist name length
const ARTIST_NAME_LENGTH: usize = 20;
// User name length
const USER_NAME_LENGTH: usize = 20;
// Audio name length
const AUDIO_NAME_LENGTH: usize = 20;
const SUBSCRIPTION_DURATION_LENGTH: usize = 10;
// Artist profile url length
const ARTIST_URL_LENGTH: usize = 255;
// User profile url length
const USER_URL_LENGTH: usize = 255;
// Audio url length
const AUDIO_URL_LENGTH: usize = 255;
const NUMBER_OF_ALLOWED_RINGBACKTONES_SPACE: usize = 5;
const NUMBER_OF_ALLOWED_SUBSCRIBERS_SPACE: usize = 5;

#[program]
mod ring_back_tone_program {
    use super::*;

    pub fn setup_platform(
        ctx: Context<RingBackTonePlatform>
    ) -> Result<()> {
        let mobile_network_operator = &mut ctx.accounts.mobile_network_operator;
        mobile_network_operator.signer = ctx.accounts.signer.key();
        // Set ringback tones count as 0 at this point
        mobile_network_operator.ring_back_tones_count = 0;
        Ok(())
    }

    pub fn sign_up_music_artist(
        ctx: Context<SignUpMusicArtist>,
        name: String,
        profile_url: String,
    ) -> Result<()> {
        if name.trim().is_empty() || profile_url.trim().is_empty() {
          return Err(Errors::CannotSignUpUser.into());
        }
        if name.as_bytes().len() > ARTIST_NAME_LENGTH {
            return Err(Errors::ExceededNameMaxLength.into());
        }
        if profile_url.as_bytes().len() > ARTIST_URL_LENGTH {
            return Err(Errors::ExceededUserUrlMaxLength.into());
        }
        let music_artist = &mut ctx.accounts.music_artist;
        music_artist.artist_wallet_address = ctx.accounts.signer.key();
        music_artist.artist_name = name;
        music_artist.artist_profile_url = profile_url;
        msg!("New music artist Added!"); //logging
        sol_log_compute_units(); //Logs how many compute units are left, important for budget
        Ok(())
    }
    
    pub fn sign_up_music_fan(
        ctx: Context<SignUpMusicFan>,
        name: String,
        profile_url: String,
    ) -> Result<()> {
        if name.trim().is_empty() || profile_url.trim().is_empty() {
          return Err(Errors::CannotSignUpUser.into());
        }
        if name.as_bytes().len() > USER_NAME_LENGTH {
            return Err(Errors::ExceededNameMaxLength.into());
        }
        if profile_url.as_bytes().len() > USER_URL_LENGTH {
            return Err(Errors::ExceededUserUrlMaxLength.into());
        }
        let music_fan = &mut ctx.accounts.music_fan;
        music_fan.user_wallet_address = ctx.accounts.signer.key();
        music_fan.user_name = name;
        music_fan.user_profile_url = profile_url;
        msg!("New music fan Added!"); //logging
        sol_log_compute_units(); //Logs how many compute units are left, important for budget
        Ok(())
    }

    pub fn upload_ring_back_tone(
        ctx: Context<UploadRingBackTone>,
        audio_name: String,
        audio_code: u8,
        audio_url: String,
        subscription_amount: u64,
        subscription_duration: String,
    ) -> Result<()> {
        msg!(&description);  //logging
        if audio_name.trim().is_empty() || audio_url.trim().is_empty() || subscription_duration.trim().is_empty() {
          return Err(Errors::CannotUploadAudio.into());
        }
        if audio_name.as_bytes().len() > AUDIO_NAME_LENGTH {
            return Err(Errors::ExceededAudioMaxLength.into());
        }
        if audio_url.as_bytes().len() > AUDIO_URL_LENGTH {
            return Err(Errors::ExceededAudioUrlMaxLength.into());
        }
        if subscription_duration.as_bytes().len() > SUBSCRIPTION_DURATION_LENGTH {
            return Err(Errors::ExceededSubscriptionDurationMaxLength.into());
        }
        let valid_audio_code = {
          if audio_code > 0 {
              true
          }
            else{false}
        };
        //  audio_code must be greater than zero
        if !valid_audio_code {
            return Err(Errors::InvalidAudioCode.into());
        }
        let valid_amount = {
          if subscription_amount > 0 {
              true
          }
            else{false}
        };
        //  withdrawal amount must be greater than zero
        if !valid_amount {
            return Err(Errors::AmountNotgreaterThanZero.into());
        }
        let mobile_network_operator = &mut ctx.accounts.mobile_network_operator;
        let ring_back_tone = &mut ctx.accounts.ring_back_tone;
        ring_back_tone.signer = ctx.accounts.signer.key();
        ring_back_tone.music_audio_name = audio_name;
        ring_back_tone.music_audio_code = audio_code;
        ring_back_tone.music_audio_url = audio_url;
        ring_back_tone.subscription_amount = subscription_amount;
        ring_back_tone.subscription_duration = subscription_duration;
        ring_back_tone.creator_time = ctx.accounts.clock.unix_timestamp;
        // Increase ring_back_tones' count by 1
        mobile_network_operator.ring_back_tones_count += 1;

        let music_artist = &mut ctx.accounts.music_artist;
        let mut iter = music_artist.ring_back_tones.iter();
        if iter.any(|&v| v == ring_back_tone) {
            return Err(Errors::CannotAddRingbackTone.into());
        }
        music_artist.ring_back_tones.push(ring_back_tone);
        
        msg!("New ringback tone Added!");  //logging
        sol_log_compute_units(); //Logs how many compute units are left, important for budget
        Ok(())
    }

    pub fn subscribe_ring_back_tone(ctx: Context<SubscribeRingBackTone>, amount: u64) -> Result<()> {
        let valid_amount = {
          if amount > 0 {
              true
          }
            else{false}
        };
        //  amount must be greater than zero
        if !valid_amount {
            return Err(Errors::AmountNotgreaterThanZero.into());
        }
        //  donation target amount cannot be exceeded
        let subscription_amount = &ctx.accounts.ring_back_tone.subscription_amount;
        let total_amount_donated  = &ctx.accounts.ring_back_tone.amount_donated;
        if amount == *subscription_amount {
            return Err(Errors::ExceededTargetAmount.into());
        }
        let instruction = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.user.key(),
            &ctx.accounts.ring_back_tone.key(),
            amount
        );
        anchor_lang::solana_program::program::invoke(
            &instruction,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.ring_back_tone.to_account_info(),
            ]
        );
        let music_artist = &mut ctx.accounts.music_artist;
        let mut iter = music_artist.subscribers.iter();
        let subscriber = ctx.accounts.signer.key();
        if iter.any(|&v| v == subscriber) {
            return Err(Errors::UserSubscribedAudio.into());
        }
        music_artist.subscribers.push(subscriber);
        music_artist.subscriptions_count += 1;
        music_artist.amount_paid += amount;
        Ok(())
    }
    
}

/// RingBackTonePlatform context
#[derive(Accounts)]
pub struct RingBackTonePlatform<'info> {
    // We must specify the space in order to initialize an account.
    #[account(
        init,
        payer = signer,
        space = size_of::<MobileNetworkOperatorAccount>() + 8, seeds = [b"mobile-network-operator".as_ref()], bump
    )]
    pub mobile_network_operator: Account<'info, MobileNetworkOperatorAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

/// SignUpMusicArtist context
#[derive(Accounts)]
pub struct SignUpMusicArtist<'info> {
    #[account(
        init,
        // use string "music-artist" and signer as seeds
        seeds = [b"music-artist".as_ref(), signer.key().as_ref()],
        bump,
        payer = signer,
        space = size_of::<MusicArtistAccount>() + ARTIST_NAME_LENGTH + ARTIST_URL_LENGTH + 8 + size_of::<RingBackToneAccount>()*NUMBER_OF_ALLOWED_RINGBACKTONES_SPACE + 32*NUMBER_OF_ALLOWED_SUBSCRIBERS_SPACE 
    )]
    pub music_artist: Account<'info, MusicArtistAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    // Clock to save time
    pub clock: Sysvar<'info, Clock>,
}

/// SignUpMusicFan context
#[derive(Accounts)]
pub struct SignUpMusicFan<'info> {
    #[account(
        init,
        // use string "music-fan" and signer as seeds
        seeds = [b"music-fan".as_ref(), signer.key().as_ref()],
        bump,
        payer = signer,
        space = size_of::<MusicFanAccount>() + USER_NAME_LENGTH + USER_URL_LENGTH + 8
    )]
    pub music_fan: Account<'info, MusicFanAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    // Clock to save time
    pub clock: Sysvar<'info, Clock>,
}

/// UploadRingBackTone context
#[derive(Accounts)]
pub struct UploadRingBackTone<'info> {
    #[account(mut, seeds = [b"mobile-network-operator".as_ref()], bump)]
    pub mobile_network_operator: Account<'info, MobileNetworkOperatorAccount>,
     #[account(mut, seeds = [b"music-artist".as_ref(), signer.key().as_ref()], bump)]
    pub music_artist: Account<'info, MusicArtistAccount>,
    #[account(
        init,
        // use string "ring-back-tone" and count of ring_back tones as seeds
        seeds = [b"ring-back-tone".as_ref(), mobile_network_operator.ring_back_tones_count.to_be_bytes().as_ref()],
        bump,
        payer = signer,
        space = size_of::<RingBackToneAccount>() + AUDIO_NAME_LENGTH + AUDIO_URL_LENGTH + SUBSCRIPTION_DURATION_LENGTH +8
    )]
    pub ring_back_tone: Account<'info, RingBackToneAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    // Clock to save time
    pub clock: Sysvar<'info, Clock>,
}

#[derive(Accounts)]
pub struct SubscribeRingBackTone<'info> {
    #[account(mut, seeds = [b"music-artist".as_ref(), signer.key().as_ref()], bump)]
    pub music_artist: Account<'info, MusicArtistAccount>,
    pub ring_back_tone: Account<'info, RingBackToneAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Mobile Network Operator Account Structure
#[account]
pub struct MobileNetworkOperatorAccount {
    // Signer address
    pub signer: Pubkey,
    // ringback tones count
    pub ring_back_tones_count: u8,
}

// Music Artist Account Structure
#[account]
pub struct MusicArtistAccount {
    pub artist_wallet_address: Pubkey,
    pub artist_name: String,
    pub artist_profile_url: String,
    pub ring_back_tones: Vec<RingBackToneAccount>,
    pub subscribers: Vec<Pubkey>,
    pub subscriptions_count: u8,
    pub amount_paid: u64,
}

// RingBack Tone Account Structure
#[account]
pub struct RingBackToneAccount {
    // Signer address
    pub signer: Pubkey,
    pub music_audio_name: String,
    pub music_audio_code: u8,
    pub music_audio_url: String,
    pub subscription_amount: u64,
    pub subscription_duration: String,
    pub creator_time: i64,
}

// Music Fan Account Structure
#[account]
pub struct MusicFanAccount {
    pub user_name: String,
    pub user_wallet_address: Pubkey,
    // user profile url
    pub user_profile_url: String,
    // ringback tone subscribed to by the music fan
    pub ring_back_tone: RingBackTone,
}

#[error_code]
pub enum Errors {
    #[msg("User cannot be signed up, missing data")]
    CannotSignUpUser,

    #[msg("Audio cannot be created, missing data")]
    CannotUploadAudio,

    #[msg("Cannot receive more than 5 likes")]
    ReachedMaxLikes,

    #[msg("Cannot follow more than 5 people")]
    ReachedMaxFollowing,

    #[msg("Exceeded name max length")]
    ExceededNameMaxLength,

    #[msg("Exceeded subscription duration max length")]
    ExceededSubscriptionDurationMaxLength,

    #[msg("Exceeded user url max length")]
    ExceededUserUrlMaxLength,

    #[msg("Exceeded audio max length")]
    ExceededAudioMaxLength,

    #[msg("Exceeded audio url max length")]
    ExceededAudioUrlMaxLength,

    #[msg("Donation target amount Exceeded.")]
    AmountNotgreaterThanZero,

    #[msg("AudioCode must have a value greater zero.")]
    InvalidAudioCode,

    #[msg("User has already Subscribed")]
    UserSubscribedAudio,

    #[msg("Ringback tone has already been added")]
    CannotAddRingbackTone,
}