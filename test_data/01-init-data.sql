CREATE TABLE test_scalar_not_null (
    id SERIAL PRIMARY KEY,
    a_text TEXT NOT NULL,
    a_uuid UUID NOT NULL,
    a_bool BOOL NOT NULL,
    a_char CHAR NOT NULL,
    a_int2 INT2 NOT NULL,
    a_int4 INT4 NOT NULL,
    a_int8 INT8 NOT NULL,
    a_float4 FLOAT4 NOT NULL,
    a_float8 FLOAT8 NOT NULL,
    a_timestamptz TIMESTAMPTZ NOT NULL,
    a_date DATE NOT NULL,
    a_jsonb JSONB NOT NULL
);

CREATE TABLE test_scalar_null (
    id SERIAL PRIMARY KEY,
    a_text TEXT NULL,
    a_uuid UUID NULL,
    a_bool BOOL NULL,
    a_char CHAR NULL,
    a_int2 INT2 NULL,
    a_int4 INT4 NULL,
    a_int8 INT8 NULL,
    a_float4 FLOAT4 NULL,
    a_float8 FLOAT8 NULL,
    a_timestamptz TIMESTAMPTZ NULL,
    a_date DATE NULL,
    a_jsonb JSONB NULL
);

CREATE TABLE test_array_not_null (
    id SERIAL PRIMARY KEY,
    some_text TEXT[] NOT NULL,
    some_uuid UUID[] NOT NULL,
    some_bool BOOL[] NOT NULL,
    some_char CHAR[] NOT NULL,
    some_int2 INT2[] NOT NULL,
    some_int4 INT4[] NOT NULL,
    some_int8 INT8[] NOT NULL,
    some_float4 FLOAT4[] NOT NULL,
    some_float8 FLOAT8[] NOT NULL,
    some_timestamptz TIMESTAMPTZ[] NOT NULL,
    some_date DATE[] NOT NULL,
    some_jsonb JSONB[] NOT NULL
);

CREATE TABLE test_array_null (
    id SERIAL PRIMARY KEY,
    some_text TEXT[] NULL,
    some_uuid UUID[] NULL,
    some_bool BOOL[] NULL,
    some_char CHAR[] NULL,
    some_int2 INT2[] NULL,
    some_int4 INT4[] NULL,
    some_int8 INT8[] NULL,
    some_float4 FLOAT4[] NULL,
    some_float8 FLOAT8[] NULL,
    some_timestamptz TIMESTAMPTZ[] NULL,
    some_date DATE[] NULL,
    some_jsonb JSONB[] NULL
);

INSERT INTO test_scalar_not_null(a_text, a_uuid, a_bool, a_char, a_int2, a_int4, a_int8, a_float4, a_float8, a_timestamptz, a_date, a_jsonb) VALUES
('Hello, World!', '3c8bc504-5281-471b-bd3d-0aa82da7c6c1', true, 'a', 42, 65537, 4294967297, 3.142, 3.142, '2020-01-01T02:30+0100', '2021-01-01', '{"hello": "world"}');
