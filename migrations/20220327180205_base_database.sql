-- Add migration script here
create table users (
    username varchar(255) not null,
    password varchar(255) not null,
    last_token varchar(255),
    primary key (username)
);

create table apps (
    id int not null auto_increment,
    name varchar(255) not null,
    owner varchar(255) not null,
    foreign key (owner) references users(username),
    primary key (id)
);

create table app_tokens (
    token varchar(255) not null,
    app_id int not null,
    permit_cors boolean not null,
    subapp_name varchar(255),
    foreign key (app_id) references apps(id),
    primary key (token)
);

create table errors_level (
    id int not null auto_increment,
    name varchar(255) not null,
    app_id int not null,
    position int not null,
    color int unsigned not null,
    unique key errors_app_position (app_id, position),
    unique key errors_app_name (app_id, name),
    foreign key (app_id) references apps(id),
    primary key (id)
);


create table errors (
    id int not null auto_increment,
    app_id int not null,
    error_level_id int not null,
    message text not null,
    stack_trace text,
    created_at bigint unsigned not null,
    foreign key (app_id) references apps(id),
    foreign key (error_level_id) references errors_level(id),
    primary key (id)
)

