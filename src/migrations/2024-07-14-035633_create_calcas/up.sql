-- Your SQL goes here
CREATE TABLE calcas (
    id INTEGER PRIMARY KEY NOT NULL,
    is_visually_malnourished BOOLEAN NOT NULL,
    has_gastro_intestinal_inconsistency BOOLEAN NOT NULL,
    has_malnutrition_patology BOOLEAN NOT NULL,
    camisa_id INTEGER NOT NULL REFERENCES camisas(id)
)