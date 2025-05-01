#![cfg(test)]
use crate::{errors::GameReviewError, GameReview, GameReviewClient};
use soroban_sdk::{
    testutils::{Address as _, MockAuth, MockAuthInvoke},
    vec, Address, Bytes, Env, IntoVal,
};

// Test helper function to create a test environment with GameReview contract
fn setup_test() -> (
    Env,
    GameReviewClient<'static>,
    Address, // admin
    Address, // user1
    Address, // user2
) {
    let env = Env::default();
    let contract_id = env.register(GameReview, {});
    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);

    // Initialize the contract
    env.mock_all_auths();

    let client = GameReviewClient::new(&env, &contract_id);
    client.initialize(&admin);

    (env, client, admin, user1, user2)
}

fn create_test_comment(env: &Env, text: &str) -> Bytes {
    Bytes::from_slice(env, text.as_bytes())
}

#[test]
fn test_initialize() {
    let env = Env::default();
    let contract_id = env.register(GameReview, {});
    let admin = Address::generate(&env);

    env.mock_all_auths();

    // Initialize the contract
    let client = GameReviewClient::new(&env, &contract_id);
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
    assert_eq!(result, Err(Ok(GameReviewError::AlreadyInitialized)));
}

#[test]
fn test_change_admin() {
    let (env, client, _, _, _) = setup_test();
    let new_admin = Address::generate(&env);

    env.mock_all_auths();

    // Change admin
    let result = client.try_change_admin(&new_admin);
    assert!(result.is_ok());

    // Verify admin was changed
    assert_eq!(client.get_admin(), new_admin);
}

#[test]
fn test_add_review() {
    let (env, client, _, user1, _) = setup_test();
    let game_id = 1;
    let rating = 4;
    let comment = create_test_comment(&env, "Great game!");

    // Mock user authentication
    env.mock_auths(&[MockAuth {
        address: &user1,
        invoke: &MockAuthInvoke {
            contract: &client.address,
            fn_name: "add_review",
            args: vec![
                &env,
                user1.to_val(),
                game_id.into_val(&env),
                rating.into_val(&env),
                comment.into_val(&env),
            ],
            sub_invokes: &[],
        },
    }]);

    // Add review
    let result = client.try_add_review(&user1, &game_id, &rating, &comment);
    assert!(result.is_ok());

    // Verify review was added
    let review = client.get_review(&game_id, &user1);
    assert_eq!(review.reviewer, user1);
    assert_eq!(review.rating, rating);
    assert_eq!(review.comment, comment);

    // Verify review count
    assert_eq!(client.get_game_review_count(&game_id), 1);

    // Verify has_reviewed flag
    assert!(client.has_reviewed(&user1, &game_id));
}

#[test]
fn test_add_review_invalid_rating() {
    let (env, client, _, user1, _) = setup_test();
    let game_id = 1;
    let invalid_rating = 6; // Ratings should be 1-5
    let comment = create_test_comment(&env, "Great game!");

    // Mock user authentication
    env.mock_auths(&[MockAuth {
        address: &user1,
        invoke: &MockAuthInvoke {
            contract: &client.address,
            fn_name: "add_review",
            args: vec![
                &env,
                user1.to_val(),
                game_id.into_val(&env),
                invalid_rating.into_val(&env),
                comment.into_val(&env),
            ],
            sub_invokes: &[],
        },
    }]);

    // Add review should fail with InvalidInput error
    let result = client.try_add_review(&user1, &game_id, &invalid_rating, &comment);
    assert_eq!(result, Err(Ok(GameReviewError::InvalidInput)));
}

#[test]
fn test_add_review_duplicate() {
    let (env, client, _, user1, _) = setup_test();
    let game_id = 1;
    let rating = 4;
    let comment = create_test_comment(&env, "Great game!");

    // Mock user authentication for first review
    env.mock_auths(&[MockAuth {
        address: &user1,
        invoke: &MockAuthInvoke {
            contract: &client.address,
            fn_name: "add_review",
            args: vec![
                &env,
                user1.to_val(),
                game_id.into_val(&env),
                rating.into_val(&env),
                comment.into_val(&env),
            ],
            sub_invokes: &[],
        },
    }]);

    // Add review
    client.add_review(&user1, &game_id, &rating, &comment);

    // Mock user authentication for second review
    env.mock_auths(&[MockAuth {
        address: &user1,
        invoke: &MockAuthInvoke {
            contract: &client.address,
            fn_name: "add_review",
            args: vec![
                &env,
                user1.to_val(),
                game_id.into_val(&env),
                rating.into_val(&env),
                comment.into_val(&env),
            ],
            sub_invokes: &[],
        },
    }]);

    // Try to add second review from same user
    let result = client.try_add_review(&user1, &game_id, &rating, &comment);
    assert_eq!(result, Err(Ok(GameReviewError::UserHasReviewed)));
}

#[test]
fn test_get_reviews() {
    let (env, client, _, user1, user2) = setup_test();
    let game_id = 1;

    // Add reviews from two different users
    let comment1 = create_test_comment(&env, "Great game!");
    let comment2 = create_test_comment(&env, "Fun gameplay!");

    // Mock auth for user1
    env.mock_auths(&[MockAuth {
        address: &user1,
        invoke: &MockAuthInvoke {
            contract: &client.address,
            fn_name: "add_review",
            args: vec![
                &env,
                user1.to_val(),
                game_id.into_val(&env),
                5_u32.into_val(&env),
                comment1.into_val(&env),
            ],
            sub_invokes: &[],
        },
    }]);
    client.add_review(&user1, &game_id, &5, &comment1);

    // Mock auth for user2
    env.mock_auths(&[MockAuth {
        address: &user2,
        invoke: &MockAuthInvoke {
            contract: &client.address,
            fn_name: "add_review",
            args: vec![
                &env,
                user2.to_val(),
                game_id.into_val(&env),
                4_u32.into_val(&env),
                comment2.into_val(&env),
            ],
            sub_invokes: &[],
        },
    }]);
    client.add_review(&user2, &game_id, &4, &comment2);

    // Get all reviews
    let reviews = client.get_reviews(&game_id, &0, &10);
    assert_eq!(reviews.len(), 2);

    // Verify reviews content
    assert_eq!(reviews.get(0).unwrap().reviewer, user1);
    assert_eq!(reviews.get(0).unwrap().rating, 5);
    assert_eq!(reviews.get(1).unwrap().reviewer, user2);
    assert_eq!(reviews.get(1).unwrap().rating, 4);
}

#[test]
fn test_get_review_not_found() {
    let (_, client, _, user1, _) = setup_test();
    let game_id = 1;

    // Try to get a review that doesn't exist
    let result = client.try_get_review(&game_id, &user1);
    assert_eq!(result, Err(Ok(GameReviewError::ReviewNotFound)));
}

#[test]
fn test_game_rating_calculation() {
    let (env, client, _, user1, user2) = setup_test();
    let game_id = 1;
    let comment = create_test_comment(&env, "Test comment");

    // Add reviews with different ratings
    env.mock_auths(&[MockAuth {
        address: &user1,
        invoke: &MockAuthInvoke {
            contract: &client.address,
            fn_name: "add_review",
            args: vec![
                &env,
                user1.to_val(),
                game_id.into_val(&env),
                5_u32.into_val(&env),
                comment.into_val(&env),
            ],
            sub_invokes: &[],
        },
    }]);
    client.add_review(&user1, &game_id, &5, &comment);

    env.mock_auths(&[MockAuth {
        address: &user2,
        invoke: &MockAuthInvoke {
            contract: &client.address,
            fn_name: "add_review",
            args: vec![
                &env,
                user2.to_val(),
                game_id.into_val(&env),
                3_u32.into_val(&env),
                comment.into_val(&env),
            ],
            sub_invokes: &[],
        },
    }]);
    client.add_review(&user2, &game_id, &3, &comment);

    // Verify average rating calculation (5 + 3) / 2 = 4
    assert_eq!(client.get_game_rating(&game_id), 4);
}

#[test]
fn test_delete_review() {
    let (env, client, admin, user1, _) = setup_test();
    let game_id = 1;
    let rating = 4;
    let comment = create_test_comment(&env, "Great game!");

    // Mock user authentication
    env.mock_auths(&[MockAuth {
        address: &user1,
        invoke: &MockAuthInvoke {
            contract: &client.address,
            fn_name: "add_review",
            args: vec![
                &env,
                user1.to_val(),
                game_id.into_val(&env),
                rating.into_val(&env),
                comment.into_val(&env),
            ],
            sub_invokes: &[],
        },
    }]);

    // Add review
    client.add_review(&user1, &game_id, &rating, &comment);

    // Verify review count before deletion
    assert_eq!(client.get_game_review_count(&game_id), 1);

    // Mock admin authentication
    env.mock_auths(&[MockAuth {
        address: &admin,
        invoke: &MockAuthInvoke {
            contract: &client.address,
            fn_name: "delete_review",
            args: vec![&env, game_id.into_val(&env), 1_u32.into_val(&env)],
            sub_invokes: &[],
        },
    }]);

    // Delete review
    let result = client.try_delete_review(&game_id, &1);
    assert!(result.is_ok());

    // Verify review was deleted
    assert_eq!(client.get_game_review_count(&game_id), 0);
    assert!(!client.has_reviewed(&user1, &game_id));
}

#[test]
fn test_delete_nonexistent_review() {
    let (env, client, admin, _, _) = setup_test();
    let game_id = 1;

    // Mock admin authentication
    env.mock_auths(&[MockAuth {
        address: &admin,
        invoke: &MockAuthInvoke {
            contract: &client.address,
            fn_name: "delete_review",
            args: vec![&env, game_id.into_val(&env), 1_u32.into_val(&env)],
            sub_invokes: &[],
        },
    }]);

    // Try to delete a nonexistent review
    let result = client.try_delete_review(&game_id, &1);
    assert_eq!(result, Err(Ok(GameReviewError::ReviewNotFound)));
}

#[test]
#[should_panic(expected = "Unauthorized")]
fn test_unauthorized_delete_review() {
    let (env, client, _, user1, _) = setup_test();
    let game_id = 1;
    let rating = 4;
    let comment = create_test_comment(&env, "Great game!");

    // Mock user authentication for adding review
    env.mock_auths(&[MockAuth {
        address: &user1,
        invoke: &MockAuthInvoke {
            contract: &client.address,
            fn_name: "add_review",
            args: vec![
                &env,
                user1.to_val(),
                game_id.into_val(&env),
                rating.into_val(&env),
                comment.into_val(&env),
            ],
            sub_invokes: &[],
        },
    }]);

    // Add review
    client.add_review(&user1, &game_id, &rating, &comment);

    // Mock user authentication for trying to delete (should fail)
    env.mock_auths(&[MockAuth {
        address: &user1,
        invoke: &MockAuthInvoke {
            contract: &client.address,
            fn_name: "delete_review",
            args: vec![&env, game_id.into_val(&env), 1_u32.into_val(&env)],
            sub_invokes: &[],
        },
    }]);

    // Try to delete as non-admin (should panic with "Unauthorized")
    client.delete_review(&game_id, &1);
}
