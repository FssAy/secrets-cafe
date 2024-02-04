BEGIN TRANSACTION;
IF $mod_id.tier >= $verifier_tier THEN {
    LET $verified = RELATE ONLY $mod_id -> verified -> $post_id RETURN position;
    UPDATE $post_id SET state=$verified_state RETURN NONE;
    $verified.position
} END;
COMMIT TRANSACTION;