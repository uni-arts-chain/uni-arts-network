use sp_core::{Pair, Public, crypto::UncheckedInto, sr25519};

use uart_runtime::{
	get_all_module_accounts,
	AccountId, BalancesConfig, GenesisConfig, SessionConfig, ValidatorSetConfig, VestingConfig,
	SudoConfig, SystemConfig, WASM_BINARY, Signature, Balance, constants::currency::*,
	opaque::SessionKeys
};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{Verify, IdentifyAccount};
use sc_service::{ChainType, Properties};
use hex_literal::hex;
use sc_telemetry::TelemetryEndpoints;



const DEFAULT_PROTOCOL_ID: &str = "uart";
const TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";


// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AccountId, AccountId, AuraId, GrandpaId) {
	(
		get_account_id_from_seed::<sr25519::Public>(s),
		get_account_id_from_seed::<sr25519::Public>(s),
		get_from_seed::<AuraId>(s),
		get_from_seed::<GrandpaId>(s),
	)
}


pub fn session_keys(
	aura: AuraId,
	grandpa: GrandpaId
) -> SessionKeys {
	SessionKeys { aura, grandpa }
}

pub fn properties() -> Option<Properties> {
	let properties = serde_json::json!({
		"ss58Format": 2,
		"tokenDecimals": 12,
		"tokenSymbol": "UART",
		"uinkDecimals": 12,
		"uinkSymbol": "UINK",
	});

	serde_json::from_value(properties).ok()
}

pub fn pangu_config() -> Result<ChainSpec, String> {
	ChainSpec::from_json_bytes(&include_bytes!("../res/pangu.json")[..])
}

pub fn staging_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or("Staging wasm binary not available".to_string())?;

	let initial_authorities: Vec<(AccountId, AccountId, AuraId, GrandpaId)> = vec![
		(
			hex!("5a185b3c60676cf602eb4bf0dab183d8eb6f9f33bf8994c248d9572dcf09de5b").into(),
			hex!("5a185b3c60676cf602eb4bf0dab183d8eb6f9f33bf8994c248d9572dcf09de5b").into(),
			hex!("5a185b3c60676cf602eb4bf0dab183d8eb6f9f33bf8994c248d9572dcf09de5b").unchecked_into(),
			hex!("7c8c270600a0535b6aed2abfe13e08db6830d69a713e9d6d15403814fc3cde66").unchecked_into()
		),
		(
			hex!("72238566d0f221dc5389f933837e611e6d95863936d926c33b0c69f317da2843").into(),
			hex!("72238566d0f221dc5389f933837e611e6d95863936d926c33b0c69f317da2843").into(),
			hex!("72238566d0f221dc5389f933837e611e6d95863936d926c33b0c69f317da2843").unchecked_into(),
			hex!("3ea0940442dae4931975a9f85068e212dd18b1437381b4cbf72cd56b0761c8b4").unchecked_into()
		)
	];

	let endowed_accounts: Vec<(AccountId, Balance)> = vec![
		(hex!("5a185b3c60676cf602eb4bf0dab183d8eb6f9f33bf8994c248d9572dcf09de5b").into(), 100_000_000 * UART)
	];

	let sudo_key: AccountId = hex!("5a185b3c60676cf602eb4bf0dab183d8eb6f9f33bf8994c248d9572dcf09de5b").into();

	Ok(ChainSpec::from_genesis(
		// Name
		"Uni-Arts Staging network",
		// ID
		"uart",
		ChainType::Live,
		move || testnet_genesis(
			wasm_binary,
			// Initial PoA authorities
			initial_authorities.clone(),
			// Sudo account
			sudo_key.clone(),
			// Pre-funded accounts
			endowed_accounts.clone(),
			true,
		),
		// Bootnodes
		vec![],
		// Telemetry
		Some(
			TelemetryEndpoints::new(vec![(TELEMETRY_URL.to_string(), 0)])
				.expect("telemetry url is valid; qed"),
		),
		// Protocol ID
		Some(DEFAULT_PROTOCOL_ID),
		// Properties
		properties(),
		// Extensions
		None,
	))
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or("Development wasm binary not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || testnet_genesis(
			wasm_binary,
			// Initial PoA authorities
			vec![
				authority_keys_from_seed("Alice"),
			],
			// Sudo account
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			// Pre-funded accounts
			vec![
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_account_id_from_seed::<sr25519::Public>("Bob"),
				get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
				get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
			].iter().map(|k| (k.clone(), 100_000 * UART )).collect::<Vec<_>>(),
			true,
		),
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		properties(),
		// Extensions
		None,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or("Development wasm binary not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || testnet_genesis(
			wasm_binary,
			// Initial PoA authorities
			vec![
				authority_keys_from_seed("Alice"),
				authority_keys_from_seed("Bob"),
			],
			// Sudo account
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			// Pre-funded accounts
			vec![
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_account_id_from_seed::<sr25519::Public>("Bob"),
				get_account_id_from_seed::<sr25519::Public>("Charlie"),
				get_account_id_from_seed::<sr25519::Public>("Dave"),
				get_account_id_from_seed::<sr25519::Public>("Eve"),
				get_account_id_from_seed::<sr25519::Public>("Ferdie"),
				get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
				get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
				get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
				get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
				get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
			].iter().map(|k| (k.clone(), 100_000 * UART )).chain(
				get_all_module_accounts()
					.iter()
					.map(|x| (x.clone(), 100_000_000 * UART)),
			).collect::<Vec<_>>(),
			true,
		),
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		properties(),
		// Extensions
		None,
	))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AccountId, AccountId, AuraId, GrandpaId)>,
	root_key: AccountId,
	endowed_accounts: Vec<(AccountId, Balance)>,
	_enable_println: bool,
) -> GenesisConfig {
	GenesisConfig {
		frame_system: Some(SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		}),
		// pallet_balances: None,
		pallet_balances: Some(BalancesConfig {
			balances: endowed_accounts.iter()
				.map(|x| (x.0.clone(), x.1.clone()))
				.chain(
					get_all_module_accounts()
						.iter()
						.map(|x| (x.clone(), 100_000_000 * UART)),
				)
				.collect(),
		}),
		pallet_validator_set: Some(ValidatorSetConfig {
			validators: initial_authorities.iter().map(|x| x.0.clone()).collect::<Vec<_>>(),
		}),
		pallet_balances_Instance1: None,
		pallet_session: Some(SessionConfig {
			keys: initial_authorities.iter().map(|x| (
				x.0.clone(),
				x.1.clone(),
				session_keys(x.2.clone(), x.3.clone()),
			)).collect::<Vec<_>>(),
		}),
		pallet_aura: None,
		pallet_grandpa: None,
		pallet_sudo: Some(SudoConfig {
			// Assign network admin rights.
			key: root_key,
		}),
		pallet_vesting: Some(VestingConfig { vesting: vec![] }),
		pallet_collective_Instance1: Some(Default::default()),
		pallet_treasury: Some(Default::default()),
	}
}
