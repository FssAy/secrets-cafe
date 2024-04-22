BEGIN TRANSACTION;
LET $is_banned = (SELECT id FROM ONLY $ban_id WHERE active=true).id == $ban_id;
IF $is_banned == false THEN {
    FOR $post IN (SELECT * FROM post WHERE iph=$post_id.iph) {
        RELATE $mod_id -> rejected -> $post SET reason=$reason RETURN NONE;
        UPDATE $post SET state=$rejected_state RETURN NONE;
    };
    CREATE $ban_id;
} END;
COMMIT TRANSACTION;