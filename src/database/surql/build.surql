DEFINE NAMESPACE cafe;
USE NS cafe;
DEFINE DATABASE main;
USE DB main;

BEGIN TRANSACTION;

DEFINE TABLE error SCHEMAFULL;
DEFINE FIELD code ON TABLE error TYPE int ASSERT $value = NONE;

DEFINE TABLE position SCHEMAFULL;
DEFINE FIELD val ON TABLE position TYPE int DEFAULT 0;

LET $val = (SELECT val FROM ONLY position:current).val;
IF $val = NONE THEN (
    CREATE position:current SET val=-1
) END;

DEFINE FUNCTION fn::inc_position() {
    LET $val = (UPDATE position:current SET val += 1 RETURN val).val;
    RETURN $val[0];
};

DEFINE TABLE post SCHEMAFULL;
DEFINE FIELD content ON TABLE post TYPE string;
DEFINE FIELD at ON TABLE post TYPE datetime VALUE time::now();
DEFINE FIELD state ON TABLE post TYPE int DEFAULT 0;

DEFINE TABLE mod SCHEMAFULL;
DEFINE FIELD name ON TABLE mod TYPE string;
DEFINE FIELD phash ON TABLE mod TYPE string;
DEFINE FIELD tier ON TABLE mod TYPE int;
DEFINE INDEX i_mod_name ON TABLE mod FIELDS name UNIQUE;

DEFINE TABLE verified;
DEFINE FIELD at ON TABLE verified TYPE datetime VALUE time::now();
DEFINE FIELD position ON TABLE verified TYPE int DEFAULT fn::inc_position();
DEFINE INDEX i_verified_position ON TABLE verified FIELDS position UNIQUE;

DEFINE TABLE rejected;
DEFINE FIELD at ON TABLE rejected TYPE datetime VALUE time::now();
DEFINE FIELD reason ON TABLE rejected TYPE string;

COMMIT TRANSACTION;