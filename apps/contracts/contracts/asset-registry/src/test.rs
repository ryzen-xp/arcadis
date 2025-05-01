#![cfg(test)]
use crate::{errors::GameAssetError, GameAssetContract, GameAssetContractClient};
use soroban_sdk::{
    testutils::{Address as _, MockAuth, MockAuthInvoke},
    vec, Address, Bytes, BytesN, Env, Map,
}; // For string formatting

// Test helper function to create a test environment with GameAssetContract
fn setup_test() -> (
    Env,
    GameAssetContractClient<'static>,
    Address, // admin
    Address, // owner1
    Address, // owner2
) {
    let env = Env::default();
    let contract_id = env.register(GameAssetContract, {});
    let admin = Address::generate(&env);
    let owner1 = Address::generate(&env);
    let owner2 = Address::generate(&env);

    // Initialize the contract
    env.mock_all_auths();

    let client = GameAssetContractClient::new(&env, &contract_id);
    client.initialize(&admin);

    (env, client, admin, owner1, owner2)
}

fn create_test_byte_string(env: &Env, text: &str) -> Bytes {
    Bytes::from_slice(env, text.as_bytes())
}

fn create_test_asset_id(env: &Env, id: u32) -> BytesN<32> {
    let mut bytes = [0u8; 32];
    bytes[0..4].copy_from_slice(&id.to_be_bytes());
    BytesN::from_array(env, &bytes)
}

fn create_test_attributes(env: &Env) -> Map<Bytes, Bytes> {
    let mut attributes = Map::new(env);
    attributes.set(
        create_test_byte_string(env, "attack"),
        create_test_byte_string(env, "15"),
    );
    attributes.set(
        create_test_byte_string(env, "defense"),
        create_test_byte_string(env, "10"),
    );
    attributes.set(
        create_test_byte_string(env, "rarity"),
        create_test_byte_string(env, "epic"),
    );
    attributes
}

#[test]
fn test_initialize() {
    let env = Env::default();
    let contract_id = env.register(GameAssetContract, {});
    let admin = Address::generate(&env);

    env.mock_all_auths();

    // Initialize the contract
    let client = GameAssetContractClient::new(&env, &contract_id);
    let result = client.try_initialize(&admin);

    // Verify initialization succeeded
    assert!(result.is_ok());

    // Verify admin is set correctly
    assert_eq!(client.get_admin(), admin);
}

#[test]
fn test_initialize_already_initialized() {
    let (env, client, admin, _, _) = setup_test();

    env.mock_all_auths();

    // Try to initialize the contract again
    let result = client.try_initialize(&admin);

    // Verify initialization fails with AlreadyInitialized error
    assert_eq!(result, Err(Ok(GameAssetError::AlreadyInitialized)));
}

#[test]
fn test_change_admin() {
    let (env, client, admin, _, _) = setup_test();
    let new_admin = Address::generate(&env);

    // Mock admin authentication
    env.mock_auths(&[MockAuth {
        address: &admin,
        invoke: &MockAuthInvoke {
            contract: &client.address,
            fn_name: "change_admin",
            args: vec![&env, new_admin.to_val()],
            sub_invokes: &[],
        },
    }]);

    // Change admin
    let result = client.try_change_admin(&new_admin);
    assert!(result.is_ok());

    // Verify admin was changed
    assert_eq!(client.get_admin(), new_admin);
}

#[test]
fn test_register_asset() {
    let (env, client, admin, owner1, _) = setup_test();
    let asset_id = create_test_asset_id(&env, 1);
    let name = create_test_byte_string(&env, "Excalibur Sword");
    let item_type = create_test_byte_string(&env, "weapon");
    let attributes = create_test_attributes(&env);

    // Mock admin authentication
    env.mock_auths(&[MockAuth {
        address: &admin,
        invoke: &MockAuthInvoke {
            contract: &client.address,
            fn_name: "register_asset",
            args: vec![
                &env,
                owner1.to_val(),
                asset_id.to_val(),
                name.to_val(),
                item_type.to_val(),
                attributes.to_val(),
            ],
            sub_invokes: &[],
        },
    }]);

    // Register asset
    let result = client.try_register_asset(&owner1, &asset_id, &name, &item_type, &attributes);
    assert!(result.is_ok());

    // Verify asset exists
    assert!(client.asset_exists(&asset_id));

    // Verify asset count increased
    assert_eq!(client.get_asset_count(), 1);

    // Verify asset data
    let asset = client.get_asset(&asset_id);
    assert_eq!(asset.id, asset_id);
    assert_eq!(asset.owner, owner1);
    assert_eq!(asset.metadata.name, name);
    assert_eq!(asset.metadata.item_type, item_type);

    // Verify user assets
    let user_assets = client.get_user_assets(&owner1);
    assert_eq!(user_assets.len(), 1);
    assert_eq!(user_assets.get(0).unwrap().id, asset_id);
}

#[test]
fn test_register_duplicate_asset() {
    let (env, client, admin, owner1, _) = setup_test();
    let asset_id = create_test_asset_id(&env, 1);
    let name = create_test_byte_string(&env, "Excalibur Sword");
    let item_type = create_test_byte_string(&env, "weapon");
    let attributes = create_test_attributes(&env);

    // Mock admin authentication for first registration
    env.mock_auths(&[MockAuth {
        address: &admin,
        invoke: &MockAuthInvoke {
            contract: &client.address,
            fn_name: "register_asset",
            args: vec![
                &env,
                owner1.to_val(),
                asset_id.to_val(),
                name.to_val(),
                item_type.to_val(),
                attributes.to_val(),
            ],
            sub_invokes: &[],
        },
    }]);

    // Register asset first time
    client.register_asset(&owner1, &asset_id, &name, &item_type, &attributes);

    // Mock admin authentication for second registration
    env.mock_auths(&[MockAuth {
        address: &admin,
        invoke: &MockAuthInvoke {
            contract: &client.address,
            fn_name: "register_asset",
            args: vec![
                &env,
                owner1.to_val(),
                asset_id.to_val(),
                name.to_val(),
                item_type.to_val(),
                attributes.to_val(),
            ],
            sub_invokes: &[],
        },
    }]);

    // Try to register the same asset again
    let result = client.try_register_asset(&owner1, &asset_id, &name, &item_type, &attributes);
    assert_eq!(result, Err(Ok(GameAssetError::AssetAlreadyRegistered)));
}

#[test]
fn test_transfer_asset() {
    let (env, client, admin, owner1, owner2) = setup_test();
    let asset_id = create_test_asset_id(&env, 1);
    let name = create_test_byte_string(&env, "Excalibur Sword");
    let item_type = create_test_byte_string(&env, "weapon");
    let attributes = create_test_attributes(&env);
    let reason = create_test_byte_string(&env, "gift");

    // Register asset
    env.mock_auths(&[MockAuth {
        address: &admin,
        invoke: &MockAuthInvoke {
            contract: &client.address,
            fn_name: "register_asset",
            args: vec![
                &env,
                owner1.to_val(),
                asset_id.to_val(),
                name.to_val(),
                item_type.to_val(),
                attributes.to_val(),
            ],
            sub_invokes: &[],
        },
    }]);
    client.register_asset(&owner1, &asset_id, &name, &item_type, &attributes);

    // Mock owner1 authentication for transfer
    env.mock_auths(&[MockAuth {
        address: &owner1,
        invoke: &MockAuthInvoke {
            contract: &client.address,
            fn_name: "transfer_asset",
            args: vec![
                &env,
                owner1.to_val(),
                owner2.to_val(),
                asset_id.to_val(),
                reason.to_val(),
            ],
            sub_invokes: &[],
        },
    }]);

    // Transfer asset from owner1 to owner2
    let result = client.try_transfer_asset(&owner1, &owner2, &asset_id, &reason);
    assert!(result.is_ok());

    // Verify new owner
    let asset = client.get_asset(&asset_id);
    assert_eq!(asset.owner, owner2);

    // Verify ownership history
    let history = client.get_asset_history(&asset_id);
    assert_eq!(history.len(), 1);
    assert_eq!(history.get(0).unwrap().previous_owner, owner1);
    assert_eq!(history.get(0).unwrap().reason, reason);

    // Verify owner1 no longer has the asset
    let owner1_assets = client.get_user_assets(&owner1);
    assert_eq!(owner1_assets.len(), 0);

    // Verify owner2 now has the asset
    let owner2_assets = client.get_user_assets(&owner2);
    assert_eq!(owner2_assets.len(), 1);
}

#[test]
#[should_panic(expected = "Unauthorized")]
fn test_unauthorized_transfer() {
    let (env, client, admin, owner1, owner2) = setup_test();
    let asset_id = create_test_asset_id(&env, 1);
    let name = create_test_byte_string(&env, "Excalibur Sword");
    let item_type = create_test_byte_string(&env, "weapon");
    let attributes = create_test_attributes(&env);
    let reason = create_test_byte_string(&env, "gift");

    // Register asset
    env.mock_auths(&[MockAuth {
        address: &admin,
        invoke: &MockAuthInvoke {
            contract: &client.address,
            fn_name: "register_asset",
            args: vec![
                &env,
                owner1.to_val(),
                asset_id.to_val(),
                name.to_val(),
                item_type.to_val(),
                attributes.to_val(),
            ],
            sub_invokes: &[],
        },
    }]);
    client.register_asset(&owner1, &asset_id, &name, &item_type, &attributes);

    // Mock owner2 authentication for transfer (should fail)
    env.mock_auths(&[MockAuth {
        address: &owner2,
        invoke: &MockAuthInvoke {
            contract: &client.address,
            fn_name: "transfer_asset",
            args: vec![
                &env,
                owner1.to_val(),
                owner2.to_val(),
                asset_id.to_val(),
                reason.to_val(),
            ],
            sub_invokes: &[],
        },
    }]);

    // transfer asset as non-owner
    client.transfer_asset(&owner1, &owner2, &asset_id, &reason);
}

#[test]
fn test_update_metadata() {
    let (env, client, admin, owner1, _) = setup_test();
    let asset_id = create_test_asset_id(&env, 1);
    let name = create_test_byte_string(&env, "Excalibur Sword");
    let item_type = create_test_byte_string(&env, "weapon");
    let attributes = create_test_attributes(&env);

    // Register asset
    env.mock_auths(&[MockAuth {
        address: &admin,
        invoke: &MockAuthInvoke {
            contract: &client.address,
            fn_name: "register_asset",
            args: vec![
                &env,
                owner1.to_val(),
                asset_id.to_val(),
                name.to_val(),
                item_type.to_val(),
                attributes.to_val(),
            ],
            sub_invokes: &[],
        },
    }]);
    client.register_asset(&owner1, &asset_id, &name, &item_type, &attributes);

    // Create updated metadata
    let new_name = create_test_byte_string(&env, "Legendary Excalibur");
    let new_type = create_test_byte_string(&env, "legendary_weapon");
    let mut new_attributes = create_test_attributes(&env);
    new_attributes.set(
        create_test_byte_string(&env, "attack"),
        create_test_byte_string(&env, "25"),
    );

    // Mock owner authentication for update
    env.mock_auths(&[MockAuth {
        address: &owner1,
        invoke: &MockAuthInvoke {
            contract: &client.address,
            fn_name: "update_metadata",
            args: vec![
                &env,
                owner1.to_val(),
                asset_id.to_val(),
                new_name.to_val(),
                new_type.to_val(),
                new_attributes.to_val(),
            ],
            sub_invokes: &[],
        },
    }]);

    // Update metadata
    let result =
        client.try_update_metadata(&owner1, &asset_id, &new_name, &new_type, &new_attributes);
    assert!(result.is_ok());

    // Verify metadata changed
    let asset = client.get_asset(&asset_id);
    assert_eq!(asset.metadata.name, new_name);
    assert_eq!(asset.metadata.item_type, new_type);
    assert_eq!(
        asset
            .metadata
            .attributes
            .get(create_test_byte_string(&env, "attack"))
            .unwrap(),
        create_test_byte_string(&env, "25")
    );

    // Also test get_asset_metadata function
    let metadata = client.get_asset_metadata(&asset_id);
    assert_eq!(metadata.name, new_name);
    assert_eq!(metadata.item_type, new_type);
}

#[test]
fn test_admin_update_metadata() {
    let (env, client, admin, owner1, _) = setup_test();
    let asset_id = create_test_asset_id(&env, 1);
    let name = create_test_byte_string(&env, "Excalibur Sword");
    let item_type = create_test_byte_string(&env, "weapon");
    let attributes = create_test_attributes(&env);

    // Register asset
    env.mock_auths(&[MockAuth {
        address: &admin,
        invoke: &MockAuthInvoke {
            contract: &client.address,
            fn_name: "register_asset",
            args: vec![
                &env,
                owner1.to_val(),
                asset_id.to_val(),
                name.to_val(),
                item_type.to_val(),
                attributes.to_val(),
            ],
            sub_invokes: &[],
        },
    }]);
    client.register_asset(&owner1, &asset_id, &name, &item_type, &attributes);

    // Create updated metadata
    let new_name = create_test_byte_string(&env, "Admin Modified Excalibur");
    let new_type = create_test_byte_string(&env, "admin_modified");
    let mut new_attributes = create_test_attributes(&env);
    new_attributes.set(
        create_test_byte_string(&env, "admin_note"),
        create_test_byte_string(&env, "balanced item"),
    );

    // Mock admin authentication for update
    env.mock_auths(&[MockAuth {
        address: &admin,
        invoke: &MockAuthInvoke {
            contract: &client.address,
            fn_name: "update_metadata",
            args: vec![
                &env,
                admin.to_val(),
                asset_id.to_val(),
                new_name.to_val(),
                new_type.to_val(),
                new_attributes.to_val(),
            ],
            sub_invokes: &[],
        },
    }]);

    // Admin updates metadata
    let result =
        client.try_update_metadata(&admin, &asset_id, &new_name, &new_type, &new_attributes);
    assert!(result.is_ok());

    // Verify metadata changed
    let asset = client.get_asset(&asset_id);
    assert_eq!(asset.metadata.name, new_name);
    assert_eq!(asset.metadata.item_type, new_type);
}

#[test]
fn test_get_all_assets() {
    let (env, client, admin, owner1, _) = setup_test();

    // Register multiple assets
    for i in 1..=5 {
        let asset_id = create_test_asset_id(&env, i);
        let name_str = match i {
            1 => "Item 1",
            2 => "Item 2",
            3 => "Item 3",
            4 => "Item 4",
            _ => "Item 5",
        };
        let name = create_test_byte_string(&env, name_str);
        let item_type = create_test_byte_string(&env, "collectible");
        let attributes = create_test_attributes(&env);

        env.mock_auths(&[MockAuth {
            address: &admin,
            invoke: &MockAuthInvoke {
                contract: &client.address,
                fn_name: "register_asset",
                args: vec![
                    &env,
                    owner1.to_val(),
                    asset_id.to_val(),
                    name.to_val(),
                    item_type.to_val(),
                    attributes.to_val(),
                ],
                sub_invokes: &[],
            },
        }]);
        client.register_asset(&owner1, &asset_id, &name, &item_type, &attributes);
    }

    // Test asset count
    assert_eq!(client.get_asset_count(), 5);

    // Test pagination
    let first_page = client.get_all_assets(&0, &3);
    assert_eq!(first_page.len(), 3);

    let second_page = client.get_all_assets(&3, &3);
    assert_eq!(second_page.len(), 2);

    // Test getting all assets at once
    let all_assets = client.get_all_assets(&0, &10);
    assert_eq!(all_assets.len(), 5);
}

#[test]
fn test_get_asset_not_found() {
    let (env, client, _, _, _) = setup_test();
    let nonexistent_id = create_test_asset_id(&env, 999);

    // Try to get an asset that doesn't exist
    let result = client.try_get_asset_info(&nonexistent_id);
    assert_eq!(result, Err(Ok(GameAssetError::AssetNotFound)));

    // Check asset_exists function
    assert!(!client.asset_exists(&nonexistent_id));
}

#[test]
fn test_get_user_assets() {
    let (env, client, admin, owner1, _) = setup_test();

    // Register multiple assets for the same owner
    for i in 1..=3 {
        let asset_id = create_test_asset_id(&env, i);
        let name_str = match i {
            1 => "Item 1",
            2 => "Item 2",
            _ => "Item 3",
        };
        let name = create_test_byte_string(&env, name_str);
        let item_type = create_test_byte_string(&env, "collectible");
        let attributes = create_test_attributes(&env);

        env.mock_auths(&[MockAuth {
            address: &admin,
            invoke: &MockAuthInvoke {
                contract: &client.address,
                fn_name: "register_asset",
                args: vec![
                    &env,
                    owner1.to_val(),
                    asset_id.to_val(),
                    name.to_val(),
                    item_type.to_val(),
                    attributes.to_val(),
                ],
                sub_invokes: &[],
            },
        }]);
        client.register_asset(&owner1, &asset_id, &name, &item_type, &attributes);
    }

    // Get user's assets
    let user_assets = client.get_user_assets(&owner1);
    assert_eq!(user_assets.len(), 3);

    // Verify each asset belongs to the user
    for asset in user_assets.iter() {
        assert_eq!(asset.owner, owner1);
    }
}
