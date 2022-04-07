-- Your SQL goes here

create table daily_activity
(
    id             serial
        primary key,
    user_id        varchar(255) not null,
    activity_date  date    not null,
    total_steps    integer      not null,
    total_distance real         not null,
    calories       integer      not null
);

