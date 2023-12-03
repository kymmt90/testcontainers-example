create table users (
    id int identity(1,1) primary key,
    name varchar(255) not null,
);

create table products (
    id int identity(1,1) primary key,
    name varchar(255) not null,
    price decimal(10,2) not null
);
