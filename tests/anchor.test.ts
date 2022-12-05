// No imports needed: web3, anchor, pg and more are globally available

describe("Test ring-back-tone-program", () => {
  const [mobileNetworkOperatorAccount, _] = web3.PublicKey
      .findProgramAddressSync(
        [
          anchor.utils.bytes.utf8.encode("mobile-network-operator"),
          pg.wallet.publicKey.toBuffer()
        ],
        pg.program.programId
      );

  const [signUpArtistPDA] = web3.PublicKey
        .findProgramAddressSync(
            [
            anchor.utils.bytes.utf8.encode("music-artist"),
            pg.wallet.publicKey.toBuffer()
            ],
            pg.program.programId
        );        

  it('Should setup ringbacktone platform', async () => {
        await pg.program.methods
            .setupPlatform()
            .accounts({
                mobileNetworkOperator: mobileNetworkOperatorAccount,
                signer: pg.wallet.publicKey,
                systemProgram: web3.SystemProgram.programId,
            })
            .rpc();

        const mobileNetworkOperatorAcc = await pg.program.account.mobileNetworkOperatorAccount.fetch(
            mobileNetworkOperatorAccount,
        );
        //assert.equal(campaignAcc.name, 'test campaign');
        //assert.equal(campaignAcc.description, 'test description');
        //assert.ok(campaignAcc.targetAmount.eq(new anchor.BN(5 * anchor.web3.LAMPORTS_PER_SOL)));
        assert.ok(mobileNetworkOperatorAcc.signer.equals(pg.wallet.publicKey));
        //assert.ok(campaignAcc.amountDonated.eq(new anchor.BN(0)));
    });

     it('Should signup music artist', async () => {
        const name: string = "firstartist";
        const profileUrl: string = "https://firstartist.com";
        await pg.program.methods
            .signUpMusicArtist(name,profileUrl)
            .accounts({
                musicArtist: signUpArtistPDA,
                signer: pg.wallet.publicKey,
                systemProgram: web3.SystemProgram.programId,
            })
            .rpc();

        const ArtistInfo = await pg.program.account.musicArtistAccount.fetch(
            signUpArtistPDA,
        );
        assert.ok(ArtistInfo.signer.equals(pg.wallet.publicKey));
    });

    it('Should signup music fan', async () => {
        const [signUpFanPDA] = await web3.PublicKey
        .findProgramAddress(
            [
            anchor.utils.bytes.utf8.encode("music-fan"),
            pg.wallet.publicKey.toBuffer()
            ],
            pg.program.programId
        );
        const name: string = "firstmusicfan";
        const profileUrl: string = "https://firstmusicfan.com";
        await pg.program.methods
            .signUpMusicFan(name,profileUrl)
            .accounts({
                musicFan: signUpFanPDA,
                signer: pg.wallet.publicKey,
                systemProgram: web3.SystemProgram.programId,
            })
            .rpc();

        const MusicFanInfo = await pg.program.account.musicFanAccount.fetch(
            signUpFanPDA,
        );
        assert.ok(MusicFanInfo.signer.equals(pg.wallet.publicKey));
    });

    it('Should upload ringback tone', async () => {
        const mobileNetworkOperatorInfo = await pg.program.account.mobileNetworkOperatorAccount.fetch(mobileNetworkOperatorAccount);
        if (mobileNetworkOperatorInfo.ringBackTonesCount > 0) {
            return;
        }
        const [uploadRingBackTonePDA, _] = await web3.PublicKey
        .findProgramAddress(
            [
            anchor.utils.bytes.utf8.encode("ring-back-tone"),
            new BN(mobileNetworkOperatorInfo.ringBackTonesCount).toArrayLike(Buffer, "be", 8),
            pg.wallet.publicKey.toBuffer()
            ],
            pg.program.programId
        );

        const audioName: string = "firstmusicaudio";
        const audioCode = new anchor.BN(1);
        const audioUrl: string = "https://firstmusicaudio.com";
        const subscriptionAmount = new anchor.BN(0.1 * anchor.web3.LAMPORTS_PER_SOL);
        const subscriptionDuration: string = "weekly";
        await pg.program.methods
            .uploadRingBackTone(audioName,audioCode,audioUrl,subscriptionAmount,subscriptionDuration)
            .accounts({
                mobileNetworkOperator: mobileNetworkOperatorAccount,
                musicArtist: signUpArtistPDA,
                ringBackTone: uploadRingBackTonePDA,
                signer: pg.wallet.publicKey,
                systemProgram: web3.SystemProgram.programId,
                clock: web3.SYSVAR_CLOCK_PUBKEY,
            })
            .rpc();

        const RingBackToneInfo = await pg.program.account.ringBackToneAccount.fetch(
            uploadRingBackTonePDA,
        );
        assert.ok(RingBackToneInfo.signer.equals(pg.wallet.publicKey));
    });

    it('Should subscribe to ringback tone', async () => {
        const mobileNetworkOperatorInfo = await pg.program.account.mobileNetworkOperatorAccount.fetch(mobileNetworkOperatorAccount);
        if (mobileNetworkOperatorInfo.ringBackTonesCount > 0) {
            return;
        }
        const [uploadRingBackTonePDA, _] = await web3.PublicKey
        .findProgramAddress(
            [
            anchor.utils.bytes.utf8.encode("ring-back-tone"),
            new BN(mobileNetworkOperatorInfo.ringBackTonesCount).toArrayLike(Buffer, "be", 8),
            pg.wallet.publicKey.toBuffer()
            ],
            pg.program.programId
        );
        const subscriptionAmount = new anchor.BN(0.1 * anchor.web3.LAMPORTS_PER_SOL);
        await pg.program.methods
            .subscribeRingBackTone(subscriptionAmount)
            .accounts({
                musicArtist: signUpArtistPDA,
                ringBackTone: uploadRingBackTonePDA,
                user: pg.wallet.publicKey,
                systemProgram: web3.SystemProgram.programId,
            })
            .rpc();
            
        const RingBackToneInfo = await pg.program.account.ringBackToneAccount.fetch(
            uploadRingBackTonePDA,
        );
        assert.ok(RingBackToneInfo.subscriptionAmount.eq(subscriptionAmount));
    });

});