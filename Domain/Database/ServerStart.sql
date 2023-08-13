drop database if exists collectiondb;
create database collectiondb;
use collectiondb;

create table `Language`
(
  Id         int unsigned auto_increment primary key,
  `Language` varchar(50) character set UTF8MB4 not null,
  `Column`   char(2)                           not null
);
insert into `Language`(`Language`, `Column`)
values ("English", "EN"),
       ("Deutsch", "DE"),
       ("español", "ES"),
       ("日本語", "JA"),
       ("한국어", "KO"),
       ("中文", "ZH"),
       ("Nederlands", "NL"),
       ("dansk", "DA");
create table Translation
(
  Id       int unsigned auto_increment primary key,
  Prefered char(2),                -- Fallback
  EN       varchar(1000) not null, -- Fallback fallback
  DE       varchar(1000),
  ES       varchar(1000),
  DA       varchar(1000),
  NL       varchar(1000),
  JA       varchar(1000) character set UTF8MB4,
  KO       varchar(1000) character set UTF8MB4,
  ZH       varchar(1000) character set UTF8MB4
);
create table Relation
(
  Id         int unsigned auto_increment primary key,
  FKRelation int unsigned not null,
  foreign key (FKRelation) references Translation (Id)
);
INSERT INTO Translation (EN, DE)
VALUES ('Unkown', 'Unbekannt');
insert into Relation (FKRelation)
Values (LAST_INSERT_ID());
INSERT INTO Translation (EN, DE)
VALUES ('Prequel', 'Prequel');
insert into Relation (FKRelation)
Values (LAST_INSERT_ID());
INSERT INTO Translation (EN, DE)
VALUES ('Sequel', 'Fortsetzung');
insert into Relation (FKRelation)
Values (LAST_INSERT_ID());
INSERT INTO Translation (EN, DE)
VALUES ('Adaptation', 'Adaption');
insert into Relation (FKRelation)
Values (LAST_INSERT_ID());
create table Genre
(
  Id      int unsigned auto_increment primary key,
  FKGenre int unsigned not null,
  foreign key (FKGenre) references Translation (Id)
);
INSERT INTO Translation (EN, DE)
VALUES ('Action', 'Action');
insert into Genre (FKGenre)
Values (LAST_INSERT_ID());
INSERT INTO Translation (EN, DE)
VALUES ('Romance', 'Romanze');
insert into Genre (FKGenre)
Values (LAST_INSERT_ID());
INSERT INTO Translation (EN, DE)
VALUES ('Horror', 'Horror');
insert into Genre (FKGenre)
Values (LAST_INSERT_ID());
INSERT INTO Translation (EN, DE)
VALUES ('Comedy', 'Komödie');
insert into Genre (FKGenre)
Values (LAST_INSERT_ID());
create table `Status`
(
  Id       int unsigned auto_increment primary key,
  FKStatus int unsigned     not null,
  foreign key (FKStatus) references Translation (Id),
  `Status` tinyint unsigned not null
);
INSERT INTO Translation (EN, DE)
VALUES ('Unkown', 'Unbekannt');
insert into `Status` (FKStatus, `Status`)
Values (LAST_INSERT_ID(), 0);
INSERT INTO Translation (EN, DE)
VALUES ('Not started', 'Noch nicht gestartet');
insert into `Status` (FKStatus, `Status`)
Values (LAST_INSERT_ID(), 1);
INSERT INTO Translation (EN, DE)
VALUES ('Airing', 'Am laufen');
insert into `Status` (FKStatus, `Status`)
Values (LAST_INSERT_ID(), 2);
INSERT INTO Translation (EN, DE)
VALUES ('Publishing', 'Am veröffentlichen');
insert into `Status` (FKStatus, `Status`)
Values (LAST_INSERT_ID(), 2);
INSERT INTO Translation (EN, DE)
VALUES ('Finished', 'Abgeschlossen');
insert into `Status` (FKStatus, `Status`)
Values (LAST_INSERT_ID(), 3);
INSERT INTO Translation (EN, DE)
VALUES ('Hiatus', 'Pausiert');
insert into `Status` (FKStatus, `Status`)
Values (LAST_INSERT_ID(), 4);
create table UserStatus
(
  Id           int unsigned auto_increment primary key,
  FKUserStatus int unsigned     not null,
  foreign key (FKUserStatus) references Translation (Id),
  `Status`     tinyint unsigned not null
);
INSERT INTO Translation (EN, DE)
VALUES ('Not started', 'Noch nicht gestartet');
insert into UserStatus (FKUserStatus, `Status`)
Values (LAST_INSERT_ID(), 1);
INSERT INTO Translation (EN, DE)
VALUES ('Reading', 'Am Lesen');
insert into UserStatus (FKUserStatus, `Status`)
Values (LAST_INSERT_ID(), 2);
INSERT INTO Translation (EN, DE)
VALUES ('Playing', 'Am Spielen');
insert into UserStatus (FKUserStatus, `Status`)
Values (LAST_INSERT_ID(), 2);
INSERT INTO Translation (EN, DE)
VALUES ('Watching', 'Am Schauen');
insert into UserStatus (FKUserStatus, `Status`)
Values (LAST_INSERT_ID(), 2);
INSERT INTO Translation (EN, DE)
VALUES ('Finished', 'Abgeschlossen');
insert into UserStatus (FKUserStatus, `Status`)
Values (LAST_INSERT_ID(), 3);
INSERT INTO Translation (EN, DE)
VALUES ('Paused', 'Pausiert');
insert into UserStatus (FKUserStatus, `Status`)
Values (LAST_INSERT_ID(), 4);
create table Theme
(
  Id      int unsigned auto_increment primary key,
  FKTheme int unsigned not null,
  foreign key (FKTheme) references Translation (Id)
);
INSERT INTO Translation (EN, DE)
VALUES ('Psychological', 'Psychologisch');
insert into Theme (FKTheme)
Values (LAST_INSERT_ID());
INSERT INTO Translation (EN, DE)
VALUES ('Gore', 'Blut');
insert into Theme (FKTheme)
Values (LAST_INSERT_ID());
INSERT INTO Translation (EN, DE)
VALUES ('Mythology', 'Mythologie');
insert into Theme (FKTheme)
Values (LAST_INSERT_ID());
INSERT INTO Translation (EN, DE)
VALUES ('Military', 'Militär');
insert into Theme (FKTheme)
Values (LAST_INSERT_ID());
INSERT INTO Translation (EN, DE)
VALUES ('Sinister', 'Finster');
insert into Theme (FKTheme)
Values (LAST_INSERT_ID());
create table `Role`
(
  Id     int unsigned auto_increment primary key,
  FKRole int unsigned not null,
  foreign key (FKRole) references Translation (Id)
);
INSERT INTO Translation (EN, DE)
VALUES ('Director', 'Regisseur');
insert into `Role` (FKRole)
Values (LAST_INSERT_ID());
INSERT INTO Translation (EN, DE)
VALUES ('Artist', 'Künstler');
insert into `Role` (FKRole)
Values (LAST_INSERT_ID());
INSERT INTO Translation (EN, DE)
VALUES ('Actor', 'Schauspieler');
insert into `Role` (FKRole)
Values (LAST_INSERT_ID());
INSERT INTO Translation (EN, DE)
VALUES ('Voice Actor', 'Synchronsprecher');
insert into `Role` (FKRole)
Values (LAST_INSERT_ID());
INSERT INTO Translation (EN, DE)
VALUES ('Sound designer', 'Sounddesigner');
insert into `Role` (FKRole)
Values (LAST_INSERT_ID());
INSERT INTO Translation (EN, DE)
VALUES ('Producer', 'Produzent');
insert into `Role` (FKRole)
Values (LAST_INSERT_ID());
INSERT INTO Translation (EN, DE)
VALUES ('Writer', 'Schreiber');
insert into `Role` (FKRole)
Values (LAST_INSERT_ID());
create table `Character`
(
  Id            int unsigned auto_increment primary key,
  FKName        int unsigned not null,
  FKFirstName   int unsigned,
  FKLastName    int unsigned,
  FKDescription int unsigned,
  Birthday      date,
  Height        int unsigned,
  ImageSource   varchar(255),
  foreign key (FKName) references Translation (Id),
  foreign key (FKFirstName) references Translation (Id),
  foreign key (FKLastName) references Translation (Id),
  foreign key (FKDescription) references Translation (Id)
);
create table Person
(
  Id            int unsigned auto_increment primary key,
  `Name`        varchar(50) not null,
  FirstName     varchar(50),
  LastName      varchar(50),
  FKDescription int unsigned,
  Birthday      date,
  Height        tinyint unsigned,
  ImageSource   varchar(255),
  foreign key (FKDescription) references Translation (Id)
);
create table PersonXRole
(
  FKPerson int unsigned not null,
  FKRole   int unsigned not null,
  foreign key (FKRole) references `Role` (Id),
  foreign key (FKPerson) references Person (Id),
  primary key (FKPerson, FKRole)
);
create table Movie
(
  Id            int unsigned auto_increment primary key,
  FKName        int unsigned not null,
  FKDescription int unsigned,
  Airing        date,
  Length        smallint unsigned,
  AverageScore  decimal(5, 2) CHECK (AverageScore BETWEEN 0.99 AND 10.01),
  ImageSource   varchar(255),
  Added         date         not null DEFAULT (CURRENT_DATE),
  foreign key (FKName) references Translation (Id),
  foreign key (FKDescription) references Translation (Id)
);
create table MovieXGenre
(
  FKMovie int unsigned not null,
  FKGenre int unsigned not null,
  foreign key (FKMovie) references Movie (Id),
  foreign key (FKGenre) references Genre (Id),
  primary key (FKMovie, FKGenre)
);
create table MovieXTheme
(
  FKMovie int unsigned not null,
  FKTheme int unsigned not null,
  foreign key (FKMovie) references Movie (Id),
  foreign key (FKTheme) references Theme (Id),
  primary key (FKMovie, FKTheme)
);
create table MovieXCreator
(
  FKMovie  int unsigned not null,
  FKPerson int unsigned not null,
  FKRole   int unsigned not null,
  foreign key (FKMovie) references Movie (Id),
  foreign key (FKPerson) references Person (Id),
  foreign key (FKRole) references `Role` (Id),
  primary key (FKMovie, FKPerson, FKRole)
);

create table Album
(
  Id            int unsigned auto_increment primary key,
  `Name`        varchar(100) not null,
  FKDescription int unsigned,
  `Release`     date,
  Songs         tinyint unsigned,
  AverageScore  decimal(5, 2) CHECK (AverageScore BETWEEN 0.99 AND 10.01),
  ImageSource   varchar(255),
  Added         date         not null DEFAULT (CURRENT_DATE),
  foreign key (FKDescription) references Translation (Id)
);
create table AlbumXCreator
(
  FKAlbum  int unsigned not null,
  FKPerson int unsigned not null,
  FKRole   int unsigned not null,
  foreign key (FKAlbum) references Album (Id),
  foreign key (FKPerson) references Person (Id),
  foreign key (FKRole) references `Role` (Id),
  primary key (FKAlbum, FKPerson, FKRole)
);
create table Song
(
  Id            int unsigned auto_increment primary key,
  `Name`        varchar(100) not null,
  FKDescription int unsigned,
  FKAlbum       int unsigned not null,
  `Release`     date,
  Songs         tinyint unsigned,
  AverageScore  decimal(5, 2) CHECK (AverageScore BETWEEN 0.99 AND 10.01),
  ImageSource   varchar(255),
  Added         date         not null DEFAULT (CURRENT_DATE),
  foreign key (FKDescription) references Translation (Id),
  foreign key (FKAlbum) references Album (Id)
);
create table SongXGenre
(
  FKSong  int unsigned not null,
  FKGenre int unsigned not null,
  foreign key (FKSong) references Song (Id),
  foreign key (FKGenre) references Genre (Id),
  primary key (FKSong, FKGenre)
);
create table SongXCreator
(
  FKSong   int unsigned not null,
  FKPerson int unsigned not null,
  FKRole   int unsigned not null,
  foreign key (FKSong) references Song (Id),
  foreign key (FKPerson) references Person (Id),
  foreign key (FKRole) references `Role` (Id),
  primary key (FKSong, FKPerson, FKRole)
);
create table Manga
(
  Id            int unsigned auto_increment primary key,
  FKName        int unsigned not null,
  FKDescription int unsigned,
  PublishStart  date,
  PublishEnd    date,
  Volumes       smallint unsigned,
  Chapters      smallint unsigned,
  AverageScore  decimal(5, 2) CHECK (AverageScore BETWEEN 0.99 AND 10.01),
  FKStatus      int unsigned,
  foreign key (FKName) references Translation (Id),
  foreign key (FKDescription) references Translation (Id),
  foreign key (FKStatus) references `Status` (Id),
  ImageSource   varchar(255),
  Added         date         not null DEFAULT (CURRENT_DATE)
);
create table MangaXGenre
(
  FKManga int unsigned not null,
  FKGenre int unsigned not null,
  foreign key (FKManga) references Manga (Id),
  foreign key (FKGenre) references Genre (Id),
  primary key (FKManga, FKGenre)
);
create table MangaXTheme
(
  FKManga int unsigned not null,
  FKTheme int unsigned not null,
  foreign key (FKManga) references Manga (Id),
  foreign key (FKTheme) references Theme (Id),
  primary key (FKManga, FKTheme)
);
create table MangaXCreator
(
  FKManga  int unsigned not null,
  FKPerson int unsigned not null,
  FKRole   int unsigned not null,
  foreign key (FKManga) references Manga (Id),
  foreign key (FKPerson) references Person (Id),
  foreign key (FKRole) references `Role` (Id),
  primary key (FKManga, FKPerson, FKRole)
);
create table Anime
(
  Id            int unsigned auto_increment primary key,
  FKName        int unsigned not null,
  FKDescription int unsigned,
  AiringStart   date,
  AiringEnd     date,
  Episodes      smallint unsigned,
  Seasons       smallint unsigned,
  AverageScore  decimal(5, 2) CHECK (AverageScore BETWEEN 0.99 AND 10.01),
  FKStatus      int unsigned,
  ImageSource   varchar(255),
  Added         date         not null DEFAULT (CURRENT_DATE),
  foreign key (FKName) references Translation (Id),
  foreign key (FKDescription) references Translation (Id),
  foreign key (FKStatus) references `Status` (Id)
);
create table AnimeSeason
(
  Season        smallint unsigned not null,
  FKAnime       int unsigned      not null,
  FKTitle       int unsigned      not null,
  FKDescription int unsigned,
  Episodes      smallint          not null,
  PublishStart  date,
  PublishEnd    date,
  AverageScore  tinyint,
  foreign key (FKAnime) references Anime (Id),
  foreign key (FKTitle) references Translation (Id),
  foreign key (FKDescription) references Translation (Id),
  primary key (FKAnime, Season)
);
create table AnimeEpisode
(
  Episode       smallint unsigned not null,
  FKAnime       int unsigned      not null,
  FKAnimeSeason int unsigned,
  FKName        int unsigned      not null,
  FKDescription int unsigned,
  Length        smallint          not null,
  AiringDate    date,
  AverageScore  tinyint,
  foreign key (FKAnime) references Anime (Id),
  foreign key (FKAnimeSeason) references AnimeSeason (Id),
  foreign key (FKName) references Translation (Id),
  foreign key (FKDescription) references Translation (Id),
  primary key (FKAnime, Episode)
);
create table AnimeXCharacter
(
  FKAnime     int unsigned not null,
  FKCharacter int unsigned not null,
  foreign key (FKAnime) references Anime (Id),
  foreign key (FKCharacter) references `Character` (Id),
  primary key (FKAnime, FKCharacter)
);
create table AnimeXGenre
(
  FKAnime int unsigned not null,
  FKGenre int unsigned not null,
  foreign key (FKAnime) references Anime (Id),
  foreign key (FKGenre) references Genre (Id),
  primary key (FKAnime, FKGenre)
);
create table AnimeXTheme
(
  FKAnime int unsigned not null,
  FKTheme int unsigned not null,
  foreign key (FKAnime) references Anime (Id),
  foreign key (FKTheme) references Theme (Id),
  primary key (FKAnime, FKTheme)
);
create table AnimeXCreator
(
  FKAnime  int unsigned not null,
  FKPerson int unsigned not null,
  FKRole   int unsigned not null,
  foreign key (FKAnime) references Anime (Id),
  foreign key (FKPerson) references Person (Id),
  foreign key (FKRole) references `Role` (Id),
  primary key (FKAnime, FKPerson, FKRole)
);
create table Comic
(
  Id            int unsigned auto_increment primary key,
  FKName        int unsigned not null,
  FKDescription int unsigned,
  PublishStart  date,
  PublishEnd    date,
  Volumes       smallint unsigned,
  Chapters      smallint unsigned,
  AverageScore  decimal(5, 2) CHECK (AverageScore BETWEEN 0.99 AND 10.01),
  FKStatus      int unsigned,
  ImageSource   varchar(255),
  Added         date         not null DEFAULT (CURRENT_DATE),
  foreign key (FKName) references Translation (Id),
  foreign key (FKDescription) references Translation (Id),
  foreign key (FKStatus) references `Status` (Id)
);
create table ComicVolume
(
  Volume        smallint unsigned not null,
  FKComic       int unsigned      not null,
  FKTitle       int unsigned      not null,
  FKDescription int unsigned,
  Pages         smallint          not null,
  PublishDate   date,
  AverageScore  tinyint,
  foreign key (FKComic) references Comic (Id),
  foreign key (FKTitle) references Translation (Id),
  foreign key (FKDescription) references Translation (Id),
  primary key (FKComic, Volume)
);
create table ComicChapter
(
  Chapter       smallint unsigned not null,
  FKComic       int unsigned      not null,
  FKComicVolume int unsigned,
  FKTitle       int unsigned      not null,
  FKDescription int unsigned,
  Pages         smallint          not null,
  PublishDate   date,
  AverageScore  tinyint,
  foreign key (FKComic) references Comic (Id),
  foreign key (FKComicVolume) references ComicVolume (Id),
  foreign key (FKTitle) references Translation (Id),
  foreign key (FKDescription) references Translation (Id),
  primary key (FKComic, Chapter)
);

create table ComicXCharacter
(
  FKComic     int unsigned not null,
  FKCharacter int unsigned not null,
  foreign key (FKComic) references Comic (Id),
  foreign key (FKCharacter) references `Character` (Id),
  primary key (FKComic, FKCharacter)
);
create table ComicXGenre
(
  FKComic int unsigned not null,
  FKGenre int unsigned not null,
  foreign key (FKComic) references Comic (Id),
  foreign key (FKGenre) references Genre (Id),
  primary key (FKComic, FKGenre)
);
create table ComicXTheme
(
  FKComic int unsigned not null,
  FKTheme int unsigned not null,
  foreign key (FKComic) references Comic (Id),
  foreign key (FKTheme) references Theme (Id),
  primary key (FKComic, FkTheme)
);
create table ComicXCreator
(
  FKComic  int unsigned not null,
  FKPerson int unsigned not null,
  FKRole   int unsigned not null,
  foreign key (FKComic) references Comic (Id),
  foreign key (FKPerson) references Person (Id),
  foreign key (FKRole) references `Role` (Id),
  primary key (FKComic, FKPerson, FKRole)
);
create table Cartoon
(
  Id            int unsigned auto_increment primary key,
  FKName        int unsigned not null,
  FKDescription int unsigned,
  AiringStart   date,
  AiringEnd     date,
  AverageScore  decimal(5, 2) CHECK (AverageScore BETWEEN 0.99 AND 10.01),
  FKStatus      int unsigned,
  ImageSource   varchar(255),
  Added         date         not null DEFAULT (CURRENT_DATE),
  foreign key (FKName) references Translation (Id),
  foreign key (FKDescription) references Translation (Id),
  foreign key (FKStatus) references `Status` (Id)
);
create table CartoonSeason
(
  Season        smallint unsigned not null,
  FKCartoon     int unsigned      not null,
  FKTitle       int unsigned      not null,
  FKDescription int unsigned,
  Episodes      smallint,
  AiringStart   date,
  AiringEnd     date,
  AverageScore  tinyint,
  foreign key (FKCartoon) references Cartoon (Id),
  foreign key (FKTitle) references Translation (Id),
  foreign key (FKDescription) references Translation (Id),
  primary key (FKCartoon, Season)
);
create table CartoonEpisode
(
  Episode         smallint unsigned not null,
  FKCartoon       int unsigned      not null,
  FKCartoonSeason int unsigned,
  FKTitle         int unsigned      not null,
  FKDescription   int unsigned,
  Length          smallint,
  AiringDate      date,
  AverageScore    tinyint,
  foreign key (FKCartoon) references Cartoon (Id),
  foreign key (FKCartoonSeason) references CartoonSeason (Id),
  foreign key (FKTitle) references Translation (Id),
  foreign key (FKDescription) references Translation (Id),
  primary key (FKCartoon, Episode)
);
create table CartoonXCharacter
(
  FKCartoon   int unsigned not null,
  FKCharacter int unsigned not null,
  foreign key (FKCartoon) references Cartoon (Id),
  foreign key (FKCharacter) references `Character` (Id),
  primary key (FKCartoon, FKCharacter)
);
create table CartoonXGenre
(
  FKCartoon int unsigned not null,
  FKGenre   int unsigned not null,
  foreign key (FKCartoon) references Cartoon (Id),
  foreign key (FKGenre) references Genre (Id),
  primary key (FKCartoon, FKGenre)
);
create table CartoonXTheme
(
  FKCartoon int unsigned not null,
  FKTheme   int unsigned not null,
  foreign key (FKCartoon) references Cartoon (Id),
  foreign key (FKTheme) references Theme (Id),
  primary key (FKCartoon, FKTheme)
);
create table CartoonXCreator
(
  FKCartoon int unsigned not null,
  FKRole    int unsigned not null,
  FKPerson  int unsigned not null,
  foreign key (FKCartoon) references Cartoon (Id),
  foreign key (FKRole) references `Role` (Id),
  foreign key (FKPerson) references Person (Id),
  primary key (FKCartoon, FKRole, FKPerson)
);
create table Book
(
  Id            int unsigned auto_increment primary key,
  FKName        int unsigned  not null,
  FKDescription int unsigned,
  Chapters      tinyint unsigned,
  Pages         smallint unsigned,
  Words         int unsigned,
  PublishDate   date          not null,
  AverageScore  decimal(5, 2) not null DEFAULT 0,
  ImageSource   varchar(255),
  Added         date          not null DEFAULT (CURRENT_DATE),
  `Rank`        int unsigned  not null,
  `Popularity`  int unsigned  not null,
  foreign key (FKName) references Translation (Id),
  foreign key (FKDescription) references Translation (Id)
);
create table BookXCharacter
(
  FKBook      int unsigned not null,
  FKCharacter int unsigned not null,
  foreign key (FKBook) references Book (Id),
  foreign key (FKCharacter) references `Character` (Id),
  primary key (FKBook, FKCharacter)
);
create table BookXGenre
(
  FKBook  int unsigned not null,
  FKGenre int unsigned not null,
  foreign key (FKBook) references Book (Id),
  foreign key (FKGenre) references Genre (Id),
  primary key (FKBook, FKGenre)
);
create table BookXTheme
(
  FKBook  int unsigned not null,
  FKTheme int unsigned not null,
  foreign key (FKBook) references Book (Id),
  foreign key (FKTheme) references Theme (Id),
  primary key (FKBook, FKTheme)
);
create table BookXCreator
(
  FKBook   int unsigned not null,
  FKRole   int unsigned not null,
  FKPerson int unsigned not null,
  foreign key (FKBook) references Book (Id),
  foreign key (FKRole) references `Role` (Id),
  foreign key (FKPerson) references Person (Id),
  primary key (FKBook, FKRole, FKPerson)
);
create table TVShow
(
  Id            int unsigned auto_increment primary key,
  FKName        int unsigned not null,
  FKDescription int unsigned,
  AiringStart   date,
  AiringEnd     date,
  AverageScore  decimal(5, 2) CHECK (AverageScore BETWEEN 0.99 AND 10.01),
  FKStatus      int unsigned,
  ImageSource   varchar(255),
  Added         date         not null DEFAULT (CURRENT_DATE),
  foreign key (FKName) references Translation (Id),
  foreign key (FKDescription) references Translation (Id),
  foreign key (FKStatus) references `Status` (Id)
);
create table TVShowSeason
(
  Season        smallint unsigned not null,
  FKTVShow      int unsigned      not null,
  FKTitle       int unsigned      not null,
  FKDescription int unsigned,
  Episodes      smallint,
  AiringStart   date,
  AiringEnd     date,
  AverageScore  tinyint,
  foreign key (FKTVShow) references TVShow (Id),
  foreign key (FKTitle) references Translation (Id),
  foreign key (FKDescription) references Translation (Id),
  primary key (FKTVShow, Season)
);
create table TVShowEpisode
(
  Episode        smallint unsigned not null,
  FKTVShow       int unsigned      not null,
  FKTVShowSeason int unsigned,
  FKTitle        int unsigned      not null,
  FKDescription  int unsigned,
  Length         smallint,
  AiringDate     date,
  AverageScore   tinyint,
  foreign key (FKTVShow) references TVShow (Id),
  foreign key (FKTVShowSeason) references TVShowSeason (Id),
  foreign key (FKTitle) references Translation (Id),
  foreign key (FKDescription) references Translation (Id),
  primary key (FKTVShow, Episode)
);
create table TVShowXCharacter
(
  FKTVShow    int unsigned not null,
  FKCharacter int unsigned not null,
  foreign key (FKTVShow) references TVShow (Id),
  foreign key (FKCharacter) references `Character` (Id),
  primary key (FKTVShow, FKCharacter)
);
create table TVShowXGenre
(
  FKTVShow int unsigned not null,
  FKGenre  int unsigned not null,
  foreign key (FKTVShow) references TVShow (Id),
  foreign key (FKGenre) references Genre (Id),
  primary key (FKTVShow, FKGenre)
);
create table TVShowXTheme
(
  FKTVShow int unsigned not null,
  FKTheme  int unsigned not null,
  foreign key (FKTVShow) references TVShow (Id),
  foreign key (FKTheme) references Theme (Id),
  primary key (FKTVShow, FKTheme)
);
create table TVShowXCreator
(
  FKTVShow int unsigned not null,
  FKRole   int unsigned not null,
  FKPerson int unsigned not null,
  foreign key (FKTVShow) references TVShow (Id),
  foreign key (FKRole) references `Role` (Id),
  foreign key (FKPerson) references Person (Id),
  primary key (FKTVShow, FKRole, FKPerson)
);
create table Game
(
  Id            int unsigned auto_increment primary key,
  FKName        int unsigned not null,
  FKDescription int unsigned,
  Published     date,
  AverageScore  decimal(5, 2) CHECK (AverageScore BETWEEN 0.99 AND 10.01),
  ImageSource   varchar(255),
  Added         date         not null DEFAULT (CURRENT_DATE),
  foreign key (FKName) references Translation (Id),
  foreign key (FKDescription) references Translation (Id)
);
create table GameXCharacter
(
  FKGame      int unsigned not null,
  FKCharacter int unsigned not null,
  foreign key (FKGame) references Game (Id),
  foreign key (FKCharacter) references `Character` (Id),
  primary key (FKGame, FKCharacter)
);
create table GameXGenre
(
  FKGame  int unsigned not null,
  FKGenre int unsigned not null,
  foreign key (FKGame) references Game (Id),
  foreign key (FKGenre) references Genre (Id),
  primary key (FKGame, FKGenre)
);
create table GameXTheme
(
  FKGame  int unsigned not null,
  FKTheme int unsigned not null,
  foreign key (FKGame) references Game (Id),
  foreign key (FKTheme) references Theme (Id),
  primary key (FKGame, FKTheme)
);
create table GameXCreator
(
  FKGame   int unsigned not null,
  FKRole   int unsigned not null,
  FKPerson int unsigned not null,
  foreign key (FKGame) references Game (Id),
  foreign key (FKRole) references `Role` (Id),
  foreign key (FKPerson) references Person (Id),
  primary key (FKGame, FKRole, FKPerson)
);
create table `User`
(
  Id            int unsigned auto_increment primary key,
  `Name`        varchar(50) character set UTF8MB4 not null,
  Joined        date                              not null DEFAULT (CURRENT_DATE),
  `Description` varchar(500),
  ImageSource   varchar(255),
  Deleted       bit
);
create table Average
(
  Id             int unsigned auto_increment primary key,
  FKUser         int unsigned,
  MangaAverage   decimal(5, 2) CHECK (MangaAverage BETWEEN 0.99 AND 10.01),
  ComicAverage   decimal(5, 2) CHECK (ComicAverage BETWEEN 0.99 AND 10.01),
  TVShowAverage  decimal(5, 2) CHECK (TVShowAverage BETWEEN 0.99 AND 10.01),
  MovieAverage   decimal(5, 2) CHECK (MovieAverage BETWEEN 0.99 AND 10.01),
  AnimeAverage   decimal(5, 2) CHECK (AnimeAverage BETWEEN 0.99 AND 10.01),
  BookAverage    decimal(5, 2) CHECK (BookAverage BETWEEN 0.99 AND 10.01),
  CartoonAverage decimal(5, 2) CHECK (CartoonAverage BETWEEN 0.99 AND 10.01),
  GameAverage    decimal(5, 2) CHECK (GameAverage BETWEEN 0.99 AND 10.01),
  AlbumAverage   decimal(5, 2) CHECK (AlbumAverage BETWEEN 0.99 AND 10.01),
  SongAverage    decimal(5, 2) CHECK (SongAverage BETWEEN 0.99 AND 10.01),
  foreign key (FKUser) references `User` (Id)
);
create table UserXAlbum
(
  FKUser   int unsigned not null,
  FKAlbum  int unsigned not null,
  Favorite bit,
  Score    tinyint unsigned,
  Review   varchar(255),
  Added    date         not null DEFAULT (CURRENT_DATE),
  foreign key (FKUser) references `User` (Id),
  foreign key (FKAlbum) references Album (Id),
  primary key (FKUser, FKAlbum)
);
create table UserXSong
(
  FKUser   int unsigned not null,
  FKSong   int unsigned not null,
  Favorite bit,
  Score    tinyint unsigned,
  Review   varchar(255),
  Added    date         not null DEFAULT (CURRENT_DATE),
  foreign key (FKUser) references `User` (Id),
  foreign key (FKSong) references Song (Id),
  primary key (FKUser, FKSong)
);
create table UserXManga
(
  FKUser       int unsigned not null,
  FKManga      int unsigned not null,
  FKUserStatus int unsigned not null,
  Favorite     bit,
  Score        tinyint unsigned,
  Review       varchar(255),
  StartDate    date,
  EndDate      date,
  Chapters     smallint unsigned,
  Added        date         not null DEFAULT (CURRENT_DATE),
  foreign key (FKUser) references `User` (Id),
  foreign key (FKManga) references Manga (Id),
  foreign key (FKUserStatus) references UserStatus (Id),
  primary key (FKUser, FKManga)
);
create table UserXComic
(
  FKUser       int unsigned not null,
  FKComic      int unsigned not null,
  FKUserStatus int unsigned not null,
  Favourite    bit,
  Score        tinyint unsigned,
  Review       varchar(255),
  StartDate    date,
  FinishedDate date,
  Chapters     smallint unsigned,
  Added        date         not null DEFAULT (CURRENT_DATE),
  foreign key (FKUser) references `User` (Id),
  foreign key (FKComic) references Comic (Id),
  foreign key (FKUserStatus) references UserStatus (Id),
  primary key (FKUser, FKComic)
);
create table UserXBook
(
  FKUser       int unsigned not null,
  FKBook       int unsigned not null,
  FKUserStatus int unsigned not null,
  Favourite    bit,
  Score        tinyint unsigned,
  Review       varchar(255),
  StartDate    date,
  FinishedDate date,
  Chapters     smallint unsigned,
  Pages        smallint unsigned,
  Added        date         not null DEFAULT (CURRENT_DATE),
  foreign key (FKUser) references `User` (Id),
  foreign key (FKBook) references Book (Id),
  foreign key (FKUserStatus) references UserStatus (Id),
  primary key (FKUser, FKBook)
);
create table UserXTVShow
(
  FKUser       int unsigned not null,
  FKTVShow     int unsigned not null,
  FKUserStatus int unsigned not null,
  Favourite    bit,
  Score        tinyint unsigned,
  Review       varchar(255),
  StartDate    date,
  FinishedDate date,
  Episodes     smallint unsigned,
  Added        date         not null DEFAULT (CURRENT_DATE),
  foreign key (FKUser) references `User` (Id),
  foreign key (FKTVShow) references TVShow (Id),
  foreign key (FKUserStatus) references UserStatus (Id),
  primary key (FKUser, FKTVShow)
);
create table UserXMovie
(
  FKUser       int unsigned not null,
  FKMovie      int unsigned not null,
  FKUserStatus int unsigned not null,
  Favourite    bit,
  Score        tinyint unsigned,
  Review       varchar(255),
  WatchDate    date,
  Added        date         not null DEFAULT (CURRENT_DATE),
  foreign key (FKUser) references `User` (Id),
  foreign key (FKMovie) references TVShow (Id),
  foreign key (FKUserStatus) references UserStatus (Id),
  primary key (FKUser, FKMovie)
);
create table UserXAnime
(
  FKUser       int unsigned not null,
  FKAnime      int unsigned not null,
  FKUserStatus int unsigned not null,
  Favourite    bit,
  Score        tinyint unsigned,
  Review       varchar(255),
  StartDate    date,
  FinishedDate date,
  Episodes     smallint unsigned,
  Added        date         not null DEFAULT (CURRENT_DATE),
  foreign key (FKUser) references `User` (Id),
  foreign key (FKAnime) references Anime (Id),
  foreign key (FKUserStatus) references UserStatus (Id),
  primary key (FKUser, FKAnime)
);
create table UserXGame
(
  FKUser       int unsigned not null,
  FKGame       int unsigned not null,
  FKUserStatus int unsigned not null,
  Favourite    bit,
  Score        tinyint unsigned,
  Review       varchar(255),
  StartDate    date,
  FinishedDate date,
  PlayTime     int unsigned,
  Added        date         not null DEFAULT (CURRENT_DATE),
  foreign key (FKUser) references `User` (Id),
  foreign key (FKGame) references Game (Id),
  foreign key (FKUserStatus) references UserStatus (Id),
  primary key (FKUser, FKGame)
);
create table Friendship
(
  Id           int unsigned auto_increment primary key,
  FKUser       int unsigned not null,
  FKSecondUser int unsigned not null,
  Added        date         not null DEFAULT (CURRENT_DATE),
  foreign key (FKUser) references `User` (Id),
  foreign key (FKSecondUser) references `User` (Id)
);
create table `Account`
(
  FKUser     int unsigned not null primary key,
  EMail      varchar(255) not null,
  `Password` char(48)     not null,
  CONSTRAINT UN_EMAIL UNIQUE (EMail),
  foreign key (FKUser) references `User` (Id)
);
create table Image
(
  Id        int unsigned auto_increment primary key,
  Url       varchar(2047) not null,
  Width     smallint      not null,
  Height    smallint      not null,
  Extension varchar(15)
);
create table ImageXTag
(
  FKImage int unsigned,
  Tag     varchar(50),
  primary key (FKImage, Tag),
  foreign key (FKImage) references `Image` (Id)
)
