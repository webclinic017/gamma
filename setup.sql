-- create main schema
CREATE SCHEMA IF NOT EXISTS history;

-- create option history table
CREATE TABLE IF NOT EXISTS history.stock_options(
	id              	BIGSERIAL PRIMARY KEY,
	symbol				VARCHAR(16) NOT NULL,
	expiration			DATE NOT NULL,
	strike				DOUBLE PRECISION NOT NULL,
	side				VARCHAR(4) NOT NULL,
	last_price			DOUBLE PRECISION NOT NULL,
	last_trade_date		DATE NOT NULL,
	bid					DOUBLE PRECISION NOT NULL,
	ask					DOUBLE PRECISION NOT NULL,
	change				DOUBLE PRECISION NOT NULL,
	percent_change		DOUBLE PRECISION NOT NULL,
	volume				INTEGER NOT NULL,
	open_interest		INTEGER NOT NULL,
	implied_volatility	DOUBLE PRECISION NOT NULL
);