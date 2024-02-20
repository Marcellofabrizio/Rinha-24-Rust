-- Coloque scripts iniciais aqui
CREATE TABLE IF NOT EXISTS clientes (
    id uuid PRIMARY KEY,
    nome VARCHAR(32) NOT NULL,
    limite INTEGER DEFAULT 0,
);

DO $$
BEGIN
  INSERT INTO clientes
  VALUES
    (gen_random_uuid(), 'o barato sai caro', 1000 * 100),
    (gen_random_uuid(), 'zan corp ltda', 800 * 100),
    (gen_random_uuid(), 'les cruders', 10000 * 100),
    (gen_random_uuid(), 'padaria joia de cocaia', 100000 * 100),
    (gen_random_uuid(), 'kid mais', 5000 * 100);
END; $$