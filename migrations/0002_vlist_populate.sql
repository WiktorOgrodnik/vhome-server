-- Add migration script here

DELETE FROM vlist;

INSERT INTO vlist (name)
VALUES
  ( 'Lista zakupów'),
  ( 'Hello, World'  );
