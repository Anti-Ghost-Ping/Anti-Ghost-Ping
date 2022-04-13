-- Add migration script here
create table guild_configs
(
    guild_id     bigint                not null
        constraint guild_configs_pk
            primary key,
    channel_id   bigint,
    everyone     boolean default false not null,
    mention_only boolean default false not null,
    color        int
);

create unique index guild_configs_guild_id_uindex
    on guild_configs (guild_id);

