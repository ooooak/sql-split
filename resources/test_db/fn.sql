CREATE OR REPLACE FUNCTION generate_and_validate_password(
  min_length INTEGER DEFAULT 8, 
  max_length INTEGER DEFAULT 16,
  require_lowercase BOOLEAN DEFAULT TRUE,
  require_uppercase BOOLEAN DEFAULT TRUE,
  require_number BOOLEAN DEFAULT TRUE,
  require_symbol BOOLEAN DEFAULT TRUE
) RETURNS TEXT AS $$
DECLARE
  alphabet TEXT := '';
  password TEXT := '';
BEGIN
  IF require_lowercase THEN alphabet := alphabet || 'abcdefghijklmnopqrstuvwxyz'; END IF;
  IF require_uppercase THEN alphabet := alphabet || 'ABCDEFGHIJKLMNOPQRSTUVWXYZ'; END IF;
  IF require_number THEN alphabet := alphabet || '0123456789'; END IF;
  IF require_symbol THEN alphabet := alphabet || '!@#$%^&*()_-+={}[]|\\:;"<>,.?/~`'; END IF;

  IF min_length < 1 THEN min_length := 1; END IF;
  IF max_length < min_length THEN max_length := min_length; END IF;

  FOR i IN 1..max_length LOOP
    password := password || substr(alphabet, floor(random() * length(alphabet) + 1), 1);
  END LOOP;

  IF length(password) < min_length THEN
    RAISE EXCEPTION 'Password length must be at least % characters', min_length;
  END IF;

  IF require_lowercase AND NOT regexp_matches(password, '[a-z]') THEN
    RAISE EXCEPTION 'Password must contain at least one lowercase letter';
  END IF;

  IF require_uppercase AND NOT regexp_matches(password, '[A-Z]') THEN
    RAISE EXCEPTION 'Password must contain at least one uppercase letter';
  END IF;

  IF require_number AND NOT regexp_matches(password, '[0-9]') THEN
    RAISE EXCEPTION 'Password must contain at least one number';
  END IF;

  IF require_symbol AND NOT regexp_matches(password, '[!@#$%^&*()_-+={}[]|\\:;"<>,.?/~`]') THEN
    RAISE EXCEPTION 'Password must contain at least one symbol';
  END IF;

  RETURN password;
END;
$$ LANGUAGE plpgsql;

