create type language as enum ( 'en','de','es','da','nl','ja','ko');
create type status as enum ('not_started','ongoing','finished','paused');
create type user_status as enum ('not_started','ongoing','finished','paused');
create table franchise
(
  id int primary key generated always as identity
);
create table rating
(
  id     int primary key generated always as identity,
  score  real,
  amount int not null default 0
);

create table franchise_translation
(
  name           varchar(50) not null,

  translation_id int         not null references franchise (id),
  language       language    not null,
  primary key (translation_id, language)
);

create table image
(
  id int primary key generated always as identity
);
create table image_data
(
  id       int primary key generated always as identity,
  image_id int           not null references image (id),
  uri      varchar(2047) not null,
  width    smallint      not null,
  height   smallint      not null
);
create table tag
(
  id   int primary key generated always as identity,
  name varchar(50) not null
);
create table image_tag
(
  image_id int references image (id),
  tag      varchar(50),
  primary key (image_id, tag)
);

create table company
(
  id      int primary key generated always as identity,
  name    varchar(100) not null,
  logo_id int          not null references image (id)
);

create table platform
(
  id         int primary key generated always as identity,
  name       varchar(50) not null,
  short_name varchar(10),
  company_id int         not null references company (id),
  logo_id    int         not null references image (id)
);

create table genre
(
  id int primary key generated always as identity
);
create table genre_translation
(
  name           varchar(50) not null,

  translation_id int         not null references genre (id),
  language       language    not null,
  primary key (translation_id, language)
);

create table theme
(
  id int primary key generated always as identity
);
create table theme_translation
(
  name           varchar(50) not null,

  translation_id int         not null references theme (id),
  language       language    not null,
  primary key (translation_id, language)
);

create table role
(
  id int primary key generated always as identity
);
create table role_translation
(
  name           varchar(50) not null,

  translation_id int         not null references role (id),
  language       language    not null,
  primary key (translation_id, language)
);

create table character
(
  id       int primary key generated always as identity,
  birthday date,
  height   int,
  image_id int references image (id)
);

create table character_translation
(
  name           varchar(150) not null,
  first_name     varchar(50),
  last_name      varchar(50),
  description    varchar(500),

  translation_id int          not null references character (id),
  language       language     not null,
  primary key (translation_id, language)
);

create table person
(
  id         int primary key generated always as identity,
  name       varchar(100) not null,
  first_name varchar(50),
  last_name  varchar(50),
  birthday   date,
  height     smallint,
  image_id   int references image (id)
);

create table person_translation
(
  description    varchar(500),

  translation_id int      not null references person (id),
  language       language not null,
  primary key (translation_id, language)
);
create table person_role
(
  person_id int not null references person (id),
  role_id   int not null references role (id),
  primary key (person_id, role_id)
);

create table movie
(
  id     int primary key generated always as identity,
  airing date,
  length interval
);
create table movie_statistic
(
  movie_id   int  not null references movie (id) primary key,
  rating_id  int  not null references rating (id),
  added      date not null default (current_date),
  rank       int  not null default 0,
  popularity int  not null default 0,
  favorites  int  not null default 0,
  members    int  not null default 0
);

create table movie_translation
(
  title          varchar(150) not null,
  description    varchar(500),
  cover_id       int          not null references image (id),

  translation_id int          not null references movie (id),
  language       language     not null,
  primary key (translation_id, language)
);

create table movie_genre
(
  movie_id int not null references movie (id),
  genre_id int not null references genre (id),
  primary key (movie_id, genre_id)
);
create table movie_theme
(
  movie_id int not null references movie (id),
  theme_id int not null references theme (id),
  primary key (movie_id, theme_id)
);
create table movie_involved
(
  movie_id  int not null references movie (id),
  person_id int not null references person (id),
  role_id   int not null references role (id),
  primary key (movie_id, person_id, role_id)
);
create table graphic_novel
(
  id            int primary key generated always as identity,
  publish_start date,
  publish_end   date,
  volumes       smallint,
  chapters      smallint,
  status        status not null
);
create table graphic_novel_statistic
(
  graphic_novel_id int  not null references graphic_novel (id) primary key,
  rating_id        int  not null references rating (id),
  added            date not null default (current_date),
  rank             int  not null default 0,
  popularity       int  not null default 0,
  favorites        int  not null default 0,
  members          int  not null default 0
);
create table graphic_novel_translation
(
  title          varchar(150) not null,
  description    varchar(500),
  cover_id       int          not null references image (id),

  translation_id int          not null references graphic_novel (id),
  language       language     not null,
  primary key (translation_id, language)
);
create table graphic_novel_volume
(
  volume           smallint unique not null,
  graphic_novel_id int             not null references graphic_novel (id),
  pages            smallint,
  published        date,
  score            smallint,
  primary key (graphic_novel_id, volume)
);
create table graphic_novel_volume_translation
(
  title          varchar(150) not null,
  description    varchar(500),

  translation_id int          not null references graphic_novel (id),
  language       language     not null,
  primary key (translation_id, language)
);
create table graphic_novel_chapter
(
  chapter                 smallint not null,
  graphic_novel_id        int      not null references graphic_novel (id),
  graphic_novel_volume_id int,
  pages                   smallint not null,
  published               date,
  score                   smallint,
  foreign key (graphic_novel_id, graphic_novel_volume_id) references graphic_novel_volume (graphic_novel_id, volume),
  primary key (graphic_novel_id, chapter)
);

create table graphic_novel_chapter_translation
(
  title                    varchar(150) not null,
  description              varchar(500),

  graphic_novel_id         int          not null references graphic_novel (id),
  graphic_novel_chapter_id int          not null,
  language                 language     not null,
  foreign key (graphic_novel_id, graphic_novel_chapter_id) references graphic_novel_chapter (graphic_novel_id, chapter),
  primary key (graphic_novel_id, graphic_novel_chapter_id, language)
);
create table graphic_novel_publisher
(
  graphic_novel_id int not null references graphic_novel (id),
  publisher_id     int not null references company (id),
  primary key (graphic_novel_id, publisher_id)
);
create table graphic_novel_character
(
  graphic_novel_id int not null references graphic_novel (id),
  character_id     int not null references character (id),
  primary key (graphic_novel_id, character_id)
);
create table graphic_novel_genre
(
  graphic_novel_id int not null references graphic_novel (id),
  genre_id         int not null references genre (id),
  primary key (graphic_novel_id, genre_id)
);
create table graphic_novel_theme
(
  graphic_novel_id int not null references graphic_novel (id),

  theme_id         int not null references theme (id),
  primary key (graphic_novel_id, theme_id)
);
create table graphic_novel_involved
(
  graphic_novel_id int not null references graphic_novel (id),
  person_id        int not null references person (id),
  role_id          int not null references role (id),
  primary key (graphic_novel_id, person_id, role_id)
);
create table book
(
  id           int primary key generated always as identity,
  published    date,
  slug         varchar(50) not null unique,

  franchise_id int references franchise (id)
);
create table book_edition
(
  id        int primary key generated always as identity,
  chapters  smallint,
  pages     smallint,
  words     int,
  published date,
  isbn13    char(13),
  language  language,
  cover_id  int not null references image (id),
  book_id   int not null references book (id)
);

create table book_edition_translation
(
  description    varchar(500),

  translation_id int      not null references book (id),
  language       language not null,
  primary key (translation_id, language)
);
create table book_edition_involved
(
  book_edition_id int not null references book_edition (id),
  role_id         int not null references role (id),
  person_id       int not null references person (id),
  primary key (book_edition_id, role_id, person_id)
);
create table book_statistic
(
  book_id    int  not null references book (id) primary key,
  rating_id  int  not null references rating (id),
  added      date not null default (current_date),
  rank       int  not null default 0,
  popularity int  not null default 0,
  favorites  int  not null default 0,
  members    int  not null default 0
);
create table book_translation
(
  title          varchar(150) not null,
  description    varchar(500),
  cover_id       int          not null references image (id),

  translation_id int          not null references book (id),
  language       language     not null,
  primary key (translation_id, language)
);
create table book_character
(
  book_id      int not null references book (id),
  character_id int not null references character (id),
  primary key (book_id, character_id)
);
create table book_image
(
  book_id  int not null references book (id),
  image_id int not null references image (id),
  primary key (book_id, image_id)
);
create table book_genre
(
  book_id  int not null references book (id),
  genre_id int not null references genre (id),
  primary key (book_id, genre_id)
);
create table book_theme
(
  book_id  int not null references book (id),
  theme_id int not null references theme (id),
  primary key (book_id, theme_id)
);
create table book_involved
(
  book_id   int not null references book (id),
  role_id   int not null references role (id),
  person_id int not null references person (id),
  primary key (book_id, role_id, person_id)
);
create table show
(
  id           int primary key generated always as identity,
  airing_start date,
  airing_end   date,
  score        real check (score between 0.99 and 10.01),
  seasons      smallint,
  status       status not null,

  franchise_id int references franchise (id)
);
create table show_statistic
(
  show_id    int  not null references show (id) primary key,
  rating_id  int  not null references rating (id),
  added      date not null default (current_date),
  rank       int  not null default 0,
  popularity int  not null default 0,
  favorites  int  not null default 0,
  members    int  not null default 0
);
create table show_translation
(
  title          varchar(150) not null,
  description    varchar(500),
  cover_id       int          not null references image (id),

  translation_id int          not null references show (id),
  language       language     not null,
  primary key (translation_id, language)
);
create table show_season
(
  season       smallint unique not null,
  show_id      int             not null references show (id),
  episodes     smallint,
  airing_start date,
  airing_end   date,
  score        smallint, --TODO: separate table
  primary key (show_id, season)
);

create table show_season_translation
(
  title          varchar(150) not null,
  description    varchar(500),

  show_id        int          not null references show (id),
  show_season_id int          not null,
  language       language     not null,
  foreign key (show_id, show_season_id) references show_season (show_id, season),
  primary key (show_id, show_season_id, language)
);
create table show_episode
(
  episode   smallint not null,
  show_id   int      not null references show (id),
  season_id int,
  length    smallint,
  airing    date,
  score     smallint,
  foreign key (show_id, season_id) references show_season (show_id, season),
  primary key (show_id, episode)
);
create table show_episode_translation
(
  title           varchar(150) not null,
  description     varchar(500),
  cover_id        int references image (id),

  show_id         int          not null references show (id),
  show_episode_id int          not null,
  language        language     not null,
  foreign key (show_id, show_episode_id) references show_episode (show_id, episode),
  primary key (show_id, show_episode_id, language)
);
create table show_character
(
  show_id      int not null references show (id),
  character_id int not null references character (id),
  primary key (show_id, character_id)
);
create table show_genre
(
  show_id  int not null references show (id),
  genre_id int not null references genre (id),
  primary key (show_id, genre_id)
);
create table show_theme
(
  show_id  int not null references show (id),
  theme_id int not null references theme (id),
  primary key (show_id, theme_id)
);
create table show_involved
(
  show_id   int not null references show (id),
  role_id   int not null references role (id),
  person_id int not null references person (id),
  primary key (show_id, role_id, person_id)
);
create table game
(
  id           int primary key generated always as identity,
  released     date,
  franchise_id int references franchise (id)
);
create table game_statistic
(
  game_id    int  not null references game (id) primary key,
  rating_id  int  not null references rating (id),
  added      date not null default (current_date),
  rank       int  not null default 0,
  popularity int  not null default 0,
  favorites  int  not null default 0,
  members    int  not null default 0
);
create table game_translation
(
  title          varchar(150) not null,
  description    varchar(500),
  cover_id       int          not null references image (id),

  translation_id int          not null references game (id),
  language       language     not null,
  primary key (translation_id, language)
);
create table game_platform
(
  game_id     int not null references game (id),
  platform_id int not null references platform (id),
  primary key (game_id, platform_id)
);
create table game_character
(
  game_id      int not null references game (id),
  character_id int not null references character (id),
  primary key (game_id, character_id)
);
create table game_genre
(
  game_id  int not null references game (id),
  genre_id int not null references genre (id),
  primary key (game_id, genre_id)
);
create table game_theme
(
  game_id  int not null references game (id),
  theme_id int not null references theme (id),
  primary key (game_id, theme_id)
);
create table game_involved
(
  game_id   int not null references game (id),
  role_id   int not null references role (id),
  person_id int not null references person (id),
  primary key (game_id, role_id, person_id)
);
create table "user"
(
  id                 int primary key generated always as identity,
  name               varchar(50) not null unique,
  joined             date        not null default (current_date),
  description        varchar(500),
  profile_picture_id int references image (id),
  deleted            boolean     not null default false
);
create table user_average
(
  user_id               int not null primary key references "user" (id),
  graphic_novel_average real check (graphic_novel_average between 0.99 and 10.01),
  show_average          real check (show_average between 0.99 and 10.01),
  movie_average         real check (movie_average between 0.99 and 10.01),
  book_average          real check (book_average between 0.99 and 10.01),
  game_average          real check (game_average between 0.99 and 10.01)
);
create table user_graphic_novel
(
  user_id          int        not null references "user" (id),
  graphic_novel_id int        not null references graphic_novel (id),
  user_status      user_status not null,
  favorite         boolean    not null,
  score            smallint,
  review           varchar(255),
  start            date,
  finished         date,
  chapters         smallint,
  added            date       not null default (current_date),
  primary key (user_id, graphic_novel_id)
);
create table user_book
(
  user_id     int        not null references "user" (id),
  book_id     int        not null references book (id),
  user_status user_status not null,
  favorite    boolean    not null,
  score       smallint,
  review      varchar(255),
  start       date,
  finished    date,
  chapters    smallint,
  pages       smallint,
  added       date       not null default (current_date),
  primary key (user_id, book_id)
);
create table user_show
(
  user_id     int        not null references "user" (id),
  show_id     int        not null references show (id),
  user_status user_status not null,
  favorite    boolean    not null,
  score       smallint,
  review      varchar(255),
  start       date,
  finished    date,
  episodes    smallint,
  added       date       not null default (current_date),
  primary key (user_id, show_id)
);
create table user_movie
(
  user_id     int        not null references "user" (id),
  movie_id    int        not null references movie (id),
  user_status user_status not null,
  favorite    boolean    not null,
  score       smallint,
  review      varchar(255),
  watched     date,
  added       date       not null default (current_date),
  primary key (user_id, movie_id)
);
create table user_game
(
  user_id     int        not null references "user" (id),
  game_id     int        not null references game (id),
  user_status user_status not null,
  favorite    boolean    not null,
  score       smallint,
  review      varchar(255),
  start       date,
  finished    date,
  play_time   int,
  added       date       not null default (current_date),
  primary key (user_id, game_id)
);
create table friendship
(
  user_id        int  not null references "user" (id),
  second_user_id int  not null references "user" (id),
  added          date not null default (current_date),
  primary key (user_id, second_user_id)
);
create table account
(
  user_id  int                 not null primary key references "user" (id),
  email    varchar(255) unique not null,
  password varchar(255)        not null
);
create index account_email_index on account using hash (email);
