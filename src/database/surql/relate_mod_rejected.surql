BEGIN TRANSACTION;
IF $post_id.state = $unverified_state THEN {
    IF $mod_id.tier >= $verifier_tier THEN {
        RELATE $mod_id -> rejected -> $post_id SET reason=$reason RETURN NONE;
        UPDATE $post_id SET state=$rejected_state RETURN NONE;
    } ELSE {
        CREATE error SET code=401;
    } END;
} ELSE {
    CREATE error SET code=409;
} END;
COMMIT TRANSACTION;