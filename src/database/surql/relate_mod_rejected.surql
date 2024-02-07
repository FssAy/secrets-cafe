BEGIN TRANSACTION;
IF $mod_id.tier >= $verifier_tier THEN {
    RELATE $mod_id -> rejected -> $post_id SET reason=$reason RETURN NONE;
    UPDATE $post_id SET state=$rejected_state RETURN NONE;
    true
} END;
COMMIT TRANSACTION;