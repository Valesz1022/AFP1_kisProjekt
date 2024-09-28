DROP TABLE IF EXISTS saved;
DROP TABLE IF EXISTS votes;
DROP TABLE IF EXISTS jokes;
DROP TABLE IF EXISTS users;

CREATE TABLE users (
    name VARCHAR(50) PRIMARY KEY,
    password VARCHAR(100) NOT NULL,
    admin BOOL NOT NULL DEFAULT FALSE
);

CREATE TABLE jokes (
    id INT PRIMARY KEY AUTO_INCREMENT,
    user_name VARCHAR(50) NOT NULL,
    content VARCHAR(500) NOT NULL UNIQUE,
    FOREIGN KEY (user_name) REFERENCES users(name) ON DELETE CASCADE
);

CREATE TABLE votes (
    user_name VARCHAR(50) NOT NULL,
    joke_id INT NOT NULL,
    vote INT NOT NULL DEFAULT 0,
    FOREIGN KEY (user_name) REFERENCES users(name) ON DELETE CASCADE,
    FOREIGN KEY (joke_id) REFERENCES jokes(id) ON DELETE CASCADE,
    PRIMARY KEY (user_name, joke_id)
);

CREATE TABLE saved (
    user_name VARCHAR(50) NOT NULL,
    joke_id INT NOT NULL,
    FOREIGN KEY (user_name) REFERENCES users(name) ON DELETE CASCADE,
    FOREIGN KEY (joke_id) REFERENCES jokes(id) ON DELETE CASCADE,
    PRIMARY KEY (user_name, joke_id)
);

SHOW TABLES;
