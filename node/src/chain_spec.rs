use sp_core::{Pair, Public, crypto::UncheckedInto, sr25519};

use uart_runtime::{
	AccountId, AuraConfig, BalancesConfig, UartConfig, UinkConfig, GenesisConfig, GrandpaConfig,
	SudoConfig, SystemConfig, WASM_BINARY, Signature, Balance, currency::*
};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{Verify, IdentifyAccount};
use sc_service::ChainType;
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
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
	(
		get_from_seed::<AuraId>(s),
		get_from_seed::<GrandpaId>(s),
	)
}

pub fn pangu_config() -> Result<ChainSpec, String> {
	ChainSpec::from_json_bytes(&include_bytes!("../res/pangu.json")[..])
}

pub fn staging_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or("Staging wasm binary not available".to_string())?;

	let properties = serde_json::json!({
		"ss58Format": 2,
    "tokenDecimals": 12,
    "tokenSymbol": "UART",
    "uinkDecimals": 12,
    "uinkSymbol": "UINK",
	});

	let initial_authorities: Vec<(
		AuraId,
		GrandpaId
	)> = vec![
		(
			hex!("80c0f5ff1e76e3980007c9ba6ce5e89a9ad5d36b0ae2afae3ade6fc63a86c952").unchecked_into(),
			hex!("15c718855b2f23f138a4d9d3182a044dc90810887590a3903fc37c49310c6712").unchecked_into()
		),

		(
			hex!("40b5ec6f41d005be33fee25b2bd069d4d149c977ef11c498384da758703d5b43").unchecked_into(),
			hex!("294cb58f14d656556ccab68af696b5f29ec92713cab60a03ad0c76ef3fccf3e9").unchecked_into()
		),
	];

	let endowed_accounts: Vec<(AccountId, Balance)> = vec![
		(hex!("80c0f5ff1e76e3980007c9ba6ce5e89a9ad5d36b0ae2afae3ade6fc63a86c952").into(), 100_000_000 * UART)
	];

	let sudo_key: AccountId = hex!("80c0f5ff1e76e3980007c9ba6ce5e89a9ad5d36b0ae2afae3ade6fc63a86c952").into();

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
		serde_json::from_value(properties).ok(),
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
		None,
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
		None,
		// Extensions
		None,
	))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AuraId, GrandpaId)>,
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
		pallet_balances: None,
		pallet_balances_Instance0: Some(UartConfig {
			balances: endowed_accounts
		}),
		pallet_balances_Instance1: None,
		pallet_aura: Some(AuraConfig {
			authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
		}),
		pallet_grandpa: Some(GrandpaConfig {
			authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect(),
		}),
		pallet_sudo: Some(SudoConfig {
			// Assign network admin rights.
			key: root_key,
		}),
	}
}
