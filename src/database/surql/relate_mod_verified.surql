BEGIN TRANSACTION;
IF $post_id.state = $unverified_state THEN {
    IF $mod_id.tier >= $verifier_tier THEN {
        LET $verified = RELATE $mod_id -> verified -> $post_id RETURN NONE;
        UPDATE $post_id SET state=$verified_state RETURN NONE;
    } ELSE {
        CREATE error SET code=401;
    } END;
} ELSE {
    CREATE error SET code=409;
} END;
COMMIT TRANSACTION;