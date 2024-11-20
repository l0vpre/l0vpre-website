create table "commissions" (
    "id"    integer not null,
    "title" text not null,
    "contact"   text not null,
    "description"   text not null,
    "status"    text not null,
    primary key("id" autoincrement)
);

create table "portfolio_categories" (
    "id" integer not null,
    "name" text not null,
    primary key("id")
);

insert into "portfolio_categories" values
    (1, 'Full'),
    (2, 'VTube'),
    (3, 'Dakimakura'),
    (4, 'Custom');

create table "portfolio_items" (
    "id" integer not null,
    "category_id" integer not null,
    "title" text not null,
    foreign key("category_id") references portfolio_categories("id"),
    primary key("id" autoincrement)
);

create table "portfolio_images" (
    "id" integer not null,
    "item_id" integer not null,
    "path" text not null,
    foreign key("item_id") references portfolio_items(id),
    primary key("id" autoincrement)
)
