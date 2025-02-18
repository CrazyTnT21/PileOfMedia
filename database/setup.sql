/*
drop database if exists collectiondb with (FORCE);
create database collectiondb;
*/
CREATE TYPE language AS ENUM ( 'EN','DE','ES','DA','NL','JA','KO');
CREATE TYPE status AS ENUM ('NotStarted','Ongoing','Finished','Paused');
CREATE TYPE userstatus AS ENUM ('NotStarted','Ongoing','Finished','Paused');
create table Franchise
(
  Id int primary key generated always as identity
);
create table Rating
(
  Id     int primary key generated always as identity,
  Score  real,
  Amount int not null default 0
);

create table FranchiseTranslation
(
  Name          varchar(50) not null,

  FKTranslation int         not null references Franchise (Id),
  Language      language    not null,
  primary key (FKTranslation, Language)
);

create table Image
(
  Id int primary key generated always as identity
);
create table ImageData
(
  Id      int primary key generated always as identity,
  FKImage int           not null references Image (Id),
  Uri     varchar(2047) not null,
  Width   smallint      not null,
  Height  smallint      not null
);
create table Tag
(
  Id   int primary key generated always as identity,
  Name varchar(50) not null
);
create table ImageTag
(
  FKImage int references Image (Id),
  Tag     varchar(50),
  primary key (FKImage, Tag)
);

create table Company
(
  Id     int primary key generated always as identity,
  Name   varchar(100) not null,
  FKLogo int          not null references Image (Id)
);

create table Platform
(
  Id        int primary key generated always as identity,
  Name      varchar(50) not null,
  ShortName varchar(10),
  FKCompany int         not null references Company (Id),
  FKLogo    int         not null references Image (Id)
);

create table Genre
(
  Id int primary key generated always as identity
);
create table GenreTranslation
(
  Name          varchar(50) not null,

  FKTranslation int         not null references Genre (Id),
  Language      language    not null,
  primary key (FKTranslation, Language)
);

create table Theme
(
  Id int primary key generated always as identity
);
create table ThemeTranslation
(
  Name          varchar(50) not null,

  FKTranslation int         not null references Theme (Id),
  Language      language    not null,
  primary key (FKTranslation, Language)
);

create table Role
(
  Id int primary key generated always as identity
);
create table RoleTranslation
(
  Name          varchar(50) not null,

  FKTranslation int         not null references Role (Id),
  Language      language    not null,
  primary key (FKTranslation, Language)
);

create table Character
(
  Id       int primary key generated always as identity,
  Birthday date,
  Height   int,
  FKImage  int references Image (Id)
);

create table CharacterTranslation
(
  Name          varchar(150) not null,
  FirstName     varchar(50),
  LastName      varchar(50),
  Description   varchar(500),

  FKTranslation int          not null references Character (Id),
  Language      language     not null,
  primary key (FKTranslation, Language)
);

create table Person
(
  Id        int primary key generated always as identity,
  Name      varchar(100) not null,
  FirstName varchar(50),
  LastName  varchar(50),
  Birthday  date,
  Height    smallint,
  FKImage   int references Image (Id)
);

create table PersonTranslation
(
  Description   varchar(500),

  FKTranslation int      not null references Person (Id),
  Language      language not null,
  primary key (FKTranslation, Language)
);
create table PersonRole
(
  FKPerson int not null references Person (Id),
  FKRole   int not null references Role (Id),
  primary key (FKPerson, FKRole)
);

create table Movie
(
  Id     int primary key generated always as identity,
  Airing date,
  Length interval
);
create table MovieStatistic
(
  FKMovie    int  not null references Movie (Id) primary key,
  FKRating   int  not null references Rating (Id),
  Added      date not null DEFAULT (CURRENT_DATE),
  Rank       int  not null default 0,
  Popularity int  not null default 0,
  Favorites  int  not null default 0,
  Members    int  not null default 0
);

create table MovieTranslation
(
  Title         varchar(150) not null,
  Description   varchar(500),
  FKCover       int          not null references Image (Id),

  FKTranslation int          not null references Movie (Id),
  Language      language     not null,
  primary key (FKTranslation, Language)
);

create table MovieGenre
(
  FKMovie int not null references Movie (Id),
  FKGenre int not null references Genre (Id),
  primary key (FKMovie, FKGenre)
);
create table MovieTheme
(
  FKMovie int not null references Movie (Id),
  FKTheme int not null references Theme (Id),
  primary key (FKMovie, FKTheme)
);
create table MovieInvolved
(
  FKMovie  int not null references Movie (Id),
  FKPerson int not null references Person (Id),
  FKRole   int not null references Role (Id),
  primary key (FKMovie, FKPerson, FKRole)
);
create table GraphicNovel
(
  Id           int primary key generated always as identity,
  PublishStart date,
  PublishEnd   date,
  Volumes      smallint,
  Chapters     smallint,
  Status       status not null
);
create table GraphicNovelStatistic
(
  FKGraphicNovel int  not null references GraphicNovel (Id) primary key,
  FKRating       int  not null references Rating (Id),
  Added          date not null DEFAULT (CURRENT_DATE),
  Rank           int  not null default 0,
  Popularity     int  not null default 0,
  Favorites      int  not null default 0,
  Members        int  not null default 0
);
create table GraphicNovelTranslation
(
  Title         varchar(150) not null,
  Description   varchar(500),
  FKCover       int          not null references Image (Id),

  FKTranslation int          not null references GraphicNovel (Id),
  Language      language     not null,
  primary key (FKTranslation, Language)
);
create table GraphicNovelVolume
(
  Volume         smallint unique not null,
  FKGraphicNovel int             not null references GraphicNovel (Id),
  Pages          smallint,
  Published      date,
  Score          smallint,
  primary key (FKGraphicNovel, Volume)
);
create table GraphicNovelVolumeTranslation
(
  Title         varchar(150) not null,
  Description   varchar(500),

  FKTranslation int          not null references GraphicNovel (Id),
  Language      language     not null,
  primary key (FKTranslation, Language)
);
create table GraphicNovelChapter
(
  Chapter              smallint not null,
  FKGraphicNovel       int      not null references GraphicNovel (Id),
  FKGraphicNovelVolume int,
  Pages                smallint not null,
  Published            date,
  Score                smallint,
  foreign key (FKGraphicNovel, FKGraphicNovelVolume) references GraphicNovelVolume (FKGraphicNovel, Volume),
  primary key (FKGraphicNovel, Chapter)
);

create table GraphicNovelChapterTranslation
(
  Title                 varchar(150) not null,
  Description           varchar(500),

  FKGraphicNovel        int          not null references GraphicNovel (Id),
  FKGraphicNovelChapter int          not null,
  Language              language     not null,
  foreign key (FKGraphicNovel, FKGraphicNovelChapter) references GraphicNovelChapter (FKGraphicNovel, Chapter),
  primary key (FKGraphicNovel, FKGraphicNovelChapter, Language)
);
create table GraphicNovelPublisher
(
  FKGraphicNovel int not null references GraphicNovel (Id),
  FKPublisher    int not null references Company (Id),
  primary key (FKGraphicNovel, FKPublisher)
);
create table GraphicNovelCharacter
(
  FKGraphicNovel int not null references GraphicNovel (Id),
  FKCharacter    int not null references Character (Id),
  primary key (FKGraphicNovel, FKCharacter)
);
create table GraphicNovelGenre
(
  FKGraphicNovel int not null references GraphicNovel (Id),
  FKGenre        int not null references Genre (Id),
  primary key (FKGraphicNovel, FKGenre)
);
create table GraphicNovelTheme
(
  FKGraphicNovel int not null references GraphicNovel (Id),

  FKTheme        int not null references Theme (Id),
  primary key (FKGraphicNovel, FkTheme)
);
create table GraphicNovelInvolved
(
  FKGraphicNovel int not null references GraphicNovel (Id),
  FKPerson       int not null references Person (Id),
  FKRole         int not null references Role (Id),
  primary key (FKGraphicNovel, FKPerson, FKRole)
);
create table Book
(
  Id          int primary key generated always as identity,
  Published   date,
  Slug        varchar(50) not null unique,

  FKFranchise int references Franchise (Id)
);
create table BookEdition
(
  Id        int primary key generated always as identity,
  Chapters  smallint,
  Pages     smallint,
  Words     int,
  Published date,
  ISBN13    char(13),
  Language  language,
  FKCover   int not null references Image (Id),
  FKBook    int not null references Book (Id)
);

create table BookEditionTranslation
(
  Description   varchar(500),

  FKTranslation int      not null references Book (Id),
  Language      language not null,
  primary key (FKTranslation, Language)
);
create table BookEditionInvolved
(
  FKBookEdition int not null references BookEdition (Id),
  FkRole        int not null references Role (Id),
  FKPerson      int not null references Person (Id),
  primary key (FKBookEdition, FKRole, FKPerson)
);
create table BookStatistic
(
  FKBook     int  not null references Book (Id) primary key,
  FKRating   int  not null references Rating (Id),
  Added      date not null DEFAULT (CURRENT_DATE),
  Rank       int  not null default 0,
  Popularity int  not null default 0,
  Favorites  int  not null default 0,
  Members    int  not null default 0
);
create table BookTranslation
(
  Title         varchar(150) not null,
  Description   varchar(500),
  FKCover       int          not null references Image (Id),

  FKTranslation int          not null references Book (Id),
  Language      language     not null,
  primary key (FKTranslation, Language)
);
create table BookCharacter
(
  FKBook      int not null references Book (Id),
  FKCharacter int not null references Character (Id),
  primary key (FKBook, FKCharacter)
);
create table BookImage
(
  FKBook  int not null references Book (Id),
  FKImage int not null references Image (Id),
  primary key (FKBook, FKImage)
);
create table BookGenre
(
  FKBook  int not null references Book (Id),
  FKGenre int not null references Genre (Id),
  primary key (FKBook, FKGenre)
);
create table BookTheme
(
  FKBook  int not null references Book (Id),
  FKTheme int not null references Theme (Id),
  primary key (FKBook, FKTheme)
);
create table BookInvolved
(
  FKBook   int not null references Book (Id),
  FKRole   int not null references Role (Id),
  FKPerson int not null references Person (Id),
  primary key (FKBook, FKRole, FKPerson)
);
create table Show
(
  Id          int primary key generated always as identity,
  AiringStart date,
  AiringEnd   date,
  Score       real CHECK (Score BETWEEN 0.99 AND 10.01),
  Seasons     smallint,
  Status      status not null,

  FKFranchise int references Franchise (Id)
);
create table ShowStatistic
(
  FKShow     int  not null references Show (Id) primary key,
  FKRating   int  not null references Rating (Id),
  Added      date not null DEFAULT (CURRENT_DATE),
  Rank       int  not null default 0,
  Popularity int  not null default 0,
  Favorites  int  not null default 0,
  Members    int  not null default 0
);
create table ShowTranslation
(
  Title         varchar(150) not null,
  Description   varchar(500),
  FKCover       int          not null references Image (Id),

  FKTranslation int          not null references Show (Id),
  Language      language     not null,
  primary key (FKTranslation, Language)
);
create table ShowSeason
(
  Season      smallint unique not null,
  FKShow      int             not null references Show (Id),
  Episodes    smallint,
  AiringStart date,
  AiringEnd   date,
  Score       smallint, --TODO: separate table
  primary key (FKShow, Season)
);

create table ShowSeasonTranslation
(
  Title        varchar(150) not null,
  Description  varchar(500),

  FKShow       int          not null references Show (Id),
  FKShowSeason int          not null,
  Language     language     not null,
  foreign key (FKShow, FKShowSeason) references ShowSeason (FKShow, Season),
  primary key (FKShow, FKShowSeason, Language)
);
create table ShowEpisode
(
  Episode  smallint not null,
  FKShow   int      not null references Show (Id),
  FKSeason int,
  Length   smallint,
  Airing   date,
  Score    smallint,
  foreign key (FKShow, FKSeason) references ShowSeason (FKShow, Season),
  primary key (FKShow, Episode)
);
create table ShowEpisodeTranslation
(
  Title         varchar(150) not null,
  Description   varchar(500),
  FKCover       int references Image (Id),

  FKShow        int          not null references Show (Id),
  FKShowEpisode int          not null,
  Language      language     not null,
  foreign key (FKShow, FKShowEpisode) references ShowEpisode (FKShow, Episode),
  primary key (FKShow, FKShowEpisode, Language)
);
create table ShowCharacter
(
  FKShow      int not null references Show (Id),
  FKCharacter int not null references Character (Id),
  primary key (FKShow, FKCharacter)
);
create table ShowGenre
(
  FKShow  int not null references Show (Id),
  FKGenre int not null references Genre (Id),
  primary key (FKShow, FKGenre)
);
create table ShowTheme
(
  FKShow  int not null references Show (Id),
  FKTheme int not null references Theme (Id),
  primary key (FKShow, FKTheme)
);
create table ShowInvolved
(
  FKShow   int not null references Show (Id),
  FKRole   int not null references Role (Id),
  FKPerson int not null references Person (Id),
  primary key (FKShow, FKRole, FKPerson)
);
create table Game
(
  Id          int primary key generated always as identity,
  Released    date,
  FKFranchise int references Franchise (Id)
);
create table GameStatistic
(
  FKGame     int  not null references Game (Id) primary key,
  FKRating   int  not null references Rating (Id),
  Added      date not null DEFAULT (CURRENT_DATE),
  Rank       int  not null default 0,
  Popularity int  not null default 0,
  Favorites  int  not null default 0,
  Members    int  not null default 0
);
create table GameTranslation
(
  Title         varchar(150) not null,
  Description   varchar(500),
  FKCover       int          not null references Image (Id),

  FKTranslation int          not null references Game (Id),
  Language      language     not null,
  primary key (FKTranslation, Language)
);
create table GamePlatform
(
  FKGame     int not null references Game (Id),
  FKPlatform int not null references Platform (Id),
  primary key (FKGame, FKPlatform)
);
create table GameCharacter
(
  FKGame      int not null references Game (Id),
  FKCharacter int not null references Character (Id),
  primary key (FKGame, FKCharacter)
);
create table GameGenre
(
  FKGame  int not null references Game (Id),
  FKGenre int not null references Genre (Id),
  primary key (FKGame, FKGenre)
);
create table GameTheme
(
  FKGame  int not null references Game (Id),
  FKTheme int not null references Theme (Id),
  primary key (FKGame, FKTheme)
);
create table GameInvolved
(
  FKGame   int not null references Game (Id),
  FKRole   int not null references Role (Id),
  FKPerson int not null references Person (Id),
  primary key (FKGame, FKRole, FKPerson)
);
create table "User"
(
  Id               int primary key generated always as identity,
  Name             varchar(50) not null unique,
  Joined           date        not null DEFAULT (CURRENT_DATE),
  Description      varchar(500),
  FKProfilePicture int references Image (Id),
  Deleted          boolean     not null DEFAULT false
);
create table UserAverage
(
  FKUser              int not null primary key references "User" (Id),
  GraphicNovelAverage real CHECK (GraphicNovelAverage BETWEEN 0.99 AND 10.01),
  ShowAverage         real CHECK (ShowAverage BETWEEN 0.99 AND 10.01),
  MovieAverage        real CHECK (MovieAverage BETWEEN 0.99 AND 10.01),
  BookAverage         real CHECK (BookAverage BETWEEN 0.99 AND 10.01),
  GameAverage         real CHECK (GameAverage BETWEEN 0.99 AND 10.01)
);
create table UserGraphicNovel
(
  FKUser         int        not null references "User" (Id),
  FKGraphicNovel int        not null references GraphicNovel (Id),
  UserStatus     userstatus not null,
  Favorite       boolean    not null,
  Score          smallint,
  Review         varchar(255),
  Start          date,
  Finished       date,
  Chapters       smallint,
  Added          date       not null DEFAULT (CURRENT_DATE),
  primary key (FKUser, FKGraphicNovel)
);
create table UserBook
(
  FKUser        int        not null references "User" (Id),
  FKBook        int        not null references Book (Id),
  UserStatus    userstatus not null,
  Favorite      boolean    not null,
  Score         smallint,
  Review        varchar(255),
  Start         date,
  Finished      date,
  Chapters      smallint,
  Pages         smallint,
  Added         date       not null DEFAULT (CURRENT_DATE),
  primary key (FKUser, FKBook)
);
create table UserShow
(
  FKUser     int        not null references "User" (Id),
  FKShow     int        not null references Show (Id),
  UserStatus userstatus not null,
  Favorite   boolean    not null,
  Score      smallint,
  Review     varchar(255),
  Start      date,
  Finished   date,
  Episodes   smallint,
  Added      date       not null DEFAULT (CURRENT_DATE),
  primary key (FKUser, FKShow)
);
create table UserMovie
(
  FKUser     int        not null references "User" (Id),
  FKMovie    int        not null references Movie (Id),
  UserStatus userstatus not null,
  Favorite   boolean    not null,
  Score      smallint,
  Review     varchar(255),
  Watched    date,
  Added      date       not null DEFAULT (CURRENT_DATE),
  primary key (FKUser, FKMovie)
);
create table UserGame
(
  FKUser     int        not null references "User" (Id),
  FKGame     int        not null references Game (Id),
  UserStatus userstatus not null,
  Favorite   boolean    not null,
  Score      smallint,
  Review     varchar(255),
  Start      date,
  Finished   date,
  PlayTime   int,
  Added      date       not null DEFAULT (CURRENT_DATE),
  primary key (FKUser, FKGame)
);
create table Friendship
(
  FKUser       int  not null references "User" (Id),
  FKSecondUser int  not null references "User" (Id),
  Added        date not null DEFAULT (CURRENT_DATE),
  primary key (FKUser, FKSecondUser)
);
create table Account
(
  FKUser   int                 not null primary key references "User" (Id),
  EMail    varchar(255) unique not null,
  Password varchar(255)        not null
);
create index AccountEmailIndex on Account using HASH (EMail);
