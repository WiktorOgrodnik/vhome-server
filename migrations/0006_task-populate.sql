-- Add migration script here

DELETE FROM vtask;
DELETE FROM vtask_assign;

INSERT INTO vtask (title, content, vlist_id, completed)
VALUES
  ( 'Pętla Kowale', 'Odwiedzić tę niesamowitą pętlę tramwajową', 1, false),
  ( 'Kładka Muchobór', 'Nowa kładka, a jeszcze tam nie byłem', 1, false),
  ( 'Park Tołpy', 'Po prostu park', 1, true ),
  ( 'Antoni Suligowski', '', 2, false ),
  ( 'Jacek Arbaz', '', 2, false ),
  ( 'Bartek Młotek', 'Przynieś ciastka', 2, false ),
  ( 'Tosia Nowak', 'Nocuje!', 2, true ),
  ( 'Masło z solą', 'Lixdark', 3, true ),
  ( 'Łosoś', 'Do ogłupiania miast', 3, true ),
  ( 'Chlebek', 'Po prostu', 3, false);

INSERT INTO vtask_assign (vtask_id, vuser_assign, assign_time)
VALUES
  ( 1, 1, NOW()),
  ( 1, 2, NOW());
