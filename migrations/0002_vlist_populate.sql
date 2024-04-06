-- Add migration script here

DELETE FROM vgroup;
DELETE FROM vlist;
DELETE FROM participation;

INSERT INTO vgroup (name)
VALUES
  ( 'Friends' ),
  ( 'Family' );

INSERT INTO vlist (group_id, name)
VALUES
  ( 1, 'Places to visit' ),
  ( 1, 'Party members' ),
  ( 2, 'Shopping list' );

INSERT INTO participation (id, name)
VALUES
  ( 1, 'Guest' ),
  ( 2, 'Member' ),
  ( 3, 'Admin' );
