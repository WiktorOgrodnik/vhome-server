# grouplist-server

## Wprowadzenie

Grouplist-server to program, który uruchomiony na serwerze będzie świadczył usługi dla klientów, którzy mają swoją grupę w tej instancji serwera

## Technologie

- rust
- postgresql
- tide

## Tabele
```
groups {
    group_id: SERIAL
    group_name: STRING
}

users {
    user_id: SERIAL
    user_name: STRING
    user_email: STRING
    user_password: STRING (SHA256)?multi_permalinks=1482935392100291&notif_id=1644537387068626&notif_t=group_highlights&ref=notif
    can_login: BOOL
    created: TIMESTAMP
    last_loging: TIMESTAMP
}

users_groups {
    user_id: EXTERNAL KEY
    group_id: EXTERNAL KEY
}

roles {
    role_id: SERIAL
    role_name: NAME
}

users_roles {
    role_id: EXTERNAL KEY
    user_id: EXTERNAL KEY
    group_id: EXTERNAL KEY
}

lists {
    list_id: SERIAL
    group_id: EXTERNAL KEY
    list_name: STRING
}

tasks {
    taks_id: SERIAL
    list_id: EXTERNAL KEY
    task_name: STRING
    task_description: STRING
    date_created: TIMESTAMP
    date_ended: TIMESTAMP
    done: BOOL
    priority_id: EXTERNAL_KEY
}

priorities {
    priority_id: SERIAL
    priority_name: STRING
    prioroty_color: HEX
}

assigns {
    user_id: EXTERNAL KEY
    task_id: EXTERNAL KEY
    assigned_by: EXTERNAL KEY
    assigned_time: TIMESTAMP
}


```
