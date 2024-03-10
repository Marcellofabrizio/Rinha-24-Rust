-- Coloque scripts iniciais aqui
CREATE TABLE IF NOT EXISTS clientes (
    id SERIAL PRIMARY KEY,
    nome VARCHAR(32) NOT NULL,
    saldo INTEGER DEFAULT 0,
    limite INTEGER DEFAULT 0
);

CREATE TABLE IF NOT EXISTS transacoes (
    id SERIAL PRIMARY KEY,
    cliente_id INTEGER NOT NULL,
    valor INTEGER NOT NULL,
    tipo CHAR(1) NOT NULL,
    descricao VARCHAR(10) NOT NULL,
    realizada_em TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_clientes_transacoes_id
		FOREIGN KEY (cliente_id) REFERENCES clientes(id)
);

CREATE FUNCTION add_debit(cliente_id INTEGER, valor INTEGER, limite INTEGER, descricao TEXT, OUT novo_saldo INTEGER) LANGUAGE plpgsql AS $$
BEGIN 

    PERFORM pg_advisory_xact_lock(cliente_id);

    UPDATE clientes SET saldo = saldo - valor
    WHERE id = cliente_id AND saldo - valor >= - limite
    RETURNING saldo INTO novo_saldo;

    IF novo_saldo IS NULL THEN
        RETURN;
    END IF;

    INSERT INTO transacoes (cliente_id, valor, limite, descricao)
    VALUES (cliente_id, valor, 'd', descricao);

END; 
$$;

CREATE FUNCTION add_credit(cliente_id INTEGER, valor INT, descricao TEXT, OUT novo_saldo INTEGER) LANGUAGE plpgsql AS $$
BEGIN
    INSERT INTO transacoes (cliente_id, valor, tipo, descricao)
    VALUES(cliente_id, valor, 'c', descricao);

    PERFORM pg_advisory_xact_lock(cliente_id);
    UPDATE clientes SET saldo = saldo + valor
    WHERE id = cliente_id
    RETURNING saldo INTO novo_saldo;
END;
$$;

CREATE FUNCTION add_transaction(cliente_id INTEGER, valor INTEGER, descricao TEXT, tipo CHAR(1), OUT novo_saldo INTEGER) 
returns integer LANGUAGE plpgsql AS $$
BEGIN
   
    IF tipo = 'd' THEN
        PERFORM limite FROM clientes WHERE id = cliente_id;

        PERFORM pg_advisory_xact_lock(cliente_id);
        UPDATE clientes SET saldo = saldo - valor
        WHERE id = cliente_id AND saldo >= - limite
        RETURNING saldo INTO novo_saldo;

        IF novo_saldo IS NULL THEN
            RETURN;
        END IF;

        INSERT INTO transacoes (cliente_id, valor, tipo, descricao)
        VALUES (cliente_id, valor, 'd', descricao);
    ELSIF tipo = 'c' THEN
        INSERT INTO transacoes (cliente_id, valor, tipo, descricao)
        VALUES(cliente_id, valor, 'c', descricao);

        PERFORM pg_advisory_xact_lock(cliente_id);
        UPDATE clientes SET saldo = saldo + valor
        WHERE id = cliente_id
        RETURNING saldo INTO novo_saldo;
    END IF;
   
   return;
END;
$$;

DO $$
BEGIN
  INSERT INTO clientes
  VALUES
    (1, 'o barato sai caro', 0, 1000 * 100),
    (2, 'zan corp ltda', 0, 800 * 100),
    (3, 'les cruders', 0, 10000 * 100),
    (4, 'padaria joia de cocaia', 0, 100000 * 100),
    (5, 'kid mais', 0, 5000 * 100);
END; 
$$;