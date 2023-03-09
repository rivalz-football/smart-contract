NETWORK=devnet


KEYPAIR_AUTH=./.keypairs/${NETWORK}/aut69244nPQ5A23MKwScxMiZxvsYeepBkNuaxK2TqSd.json
KEYPAIR_PROGRAM=./.keypairs/${NETWORK}/rivi27uFE2UGJCR2WmzddviqS6RWiTzciC8KnVp2rhi.json

env:
	solana config set --url ${NETWORK}
	solana config set --keypair  ${KEYPAIR_AUTH}
	cp ${KEYPAIR_AUTH} ~/.config/solana/id.json
	mkdir -p ./target/deploy && cp ${KEYPAIR_PROGRAM} ./target/deploy/rivalz_goalflip-keypair.json || true # this can safely fail


airdrop:
	solana --url ${NETWORK} \
	--keypair ${KEYPAIR_AUTH} \
	airdrop 2

build:
	anchor build

deploy: build
	solana program deploy ./target/deploy/rivalz_goalflip.so \
	--url ${NETWORK} \
	--program-id ${KEYPAIR_PROGRAM} \
	--keypair ${KEYPAIR_AUTH}
	say -v Samantha protocol deployment completed || true

deploy-resume: build
	solana program deploy ./target/deploy/rivalz_goalflip.so \
	--url ${NETWORK} \
	--program-id ${KEYPAIR_PROGRAM} \
	--keypair ${KEYPAIR_AUTH} \
	--buffer $(buffer)
	say -v Samantha protocol deployment completed || true

test:
	#make airdrop || true
	anchor test

test-skip:
	anchor test --skip-build --skip-deploy
