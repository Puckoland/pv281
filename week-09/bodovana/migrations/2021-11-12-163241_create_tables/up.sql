CREATE TABLE categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    parent_id INTEGER REFERENCES categories(id)
);

CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    price INTEGER NOT NULL
    -- category_id INTEGER REFERENCES categories(id)
);

CREATE TABLE categorizations (
    id SERIAL PRIMARY KEY,
    category_id INTEGER REFERENCES categories(id) NOT NULL,
    product_id INTEGER REFERENCES products(id) NOT NULL
);

CREATE TABLE address (
    id SERIAL PRIMARY KEY,
    city VARCHAR(255) NOT NULL,
    street VARCHAR(255) NOT NULL,
    number INTEGER NOT NULL,
    zip CHAR(5) NOT NULL
);

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR(64) NOT NULL,
    last_name VARCHAR(64) NOT NULL,
    address_id INTEGER REFERENCES address(id) NOT NULL
);

CREATE TABLE MyState (
    id SERIAL PRIMARY KEY,
    my_state VARCHAR(64) NOT NULL
);
INSERT INTO MyState (my_state) VALUES ('ordered');
INSERT INTO MyState (my_state) VALUES ('shipped');
INSERT INTO MyState (my_state) VALUES ('delivered');

CREATE TABLE orders (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR(64) NOT NULL,
    last_name VARCHAR(64) NOT NULL,
    city VARCHAR(255) NOT NULL,
    street VARCHAR(255) NOT NULL,
    number INTEGER NOT NULL,
    zip CHAR(5) NOT NULL,
    name VARCHAR(255) NOT NULL,
    price INTEGER NOT NULL,
    state_id INTEGER REFERENCES MyState(id) DEFAULT 1 NOT NULL
);
