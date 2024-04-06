--- Add migration script here

DELETE FROM vuser;
DELETE FROM vuser_groups;

INSERT INTO vuser (login, passwd)
VALUES
  ( 'user1', '$2b$12$iLmS6/.s.PrXYuSAZr30LOlUiu1hmQqZ9YidPWMXLJk1tLdoUVg9a' ),
  ( 'user2', '$2b$12$SPTRcKQyxD91xbPmNjNRNuxZcivy3Go7oXW9TfG8JaR60hAhDq3Mq' );

INSERT INTO vuser_groups (user_id, group_id, participation_id)
VALUES
  ( 1, 1, 2 ),
  ( 2, 2, 2 ); 
