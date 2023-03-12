start transaction;
create database collectiondb;
use collectiondb;
create table `Language`
(
  PK         int unsigned                      auto_increment primary key,
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
  PK       int unsigned auto_increment primary key,
  Prefered char(2),
  EN       varchar(1000),
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
  PK         int unsigned auto_increment primary key,
  FKRelation int unsigned not null,
  foreign key (FKRelation) references Translation (PK)
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
  PK      int unsigned auto_increment primary key,
  FKGenre int unsigned not null,
  foreign key (FKGenre) references Translation (PK)
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
  PK       int unsigned     auto_increment primary key,
  FKStatus int unsigned     not null,
  foreign key (FKStatus) references Translation (PK),
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
VALUES ('Publishing', 'Zurzeit veröffentlicht');
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
  PK           int unsigned     auto_increment primary key,
  FKUserStatus int unsigned     not null,
  foreign key (FKUserStatus) references Translation (PK),
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
  PK      int unsigned auto_increment primary key,
  FKTheme int unsigned not null,
  foreign key (FKTheme) references Translation (PK)
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
  PK     int unsigned auto_increment primary key,
  FKRole int unsigned not null,
  foreign key (FKRole) references Translation (PK)
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
  PK            int unsigned auto_increment primary key,
  FKName        int unsigned not null,
  FKFirstName   int unsigned,
  FKLastName    int unsigned,
  FKDescription int unsigned,
  Birthday      date,
  Height        int unsigned,
  ImageSource   varchar(255),
  foreign key (FKName) references Translation (PK),
  foreign key (FKFirstName) references Translation (PK),
  foreign key (FKLastName) references Translation (PK),
  foreign key (FKDescription) references Translation (PK)
);
create table Person
(
  PK            int unsigned auto_increment primary key,
  `Name`        varchar(50)  not null,
  FirstName     varchar(50),
  LastName      varchar(50),
  FKDescription int unsigned,
  Birthday      date,
  Height        tinyint unsigned,
  ImageSource   varchar(255),
  foreign key (FKDescription) references Translation (PK)
);
create table PersonXRole
(
  FKPerson int unsigned not null,
  FKRole   int unsigned not null,
  foreign key (FKRole) references `Role` (PK),
  foreign key (FKPerson) references Person (PK),
  primary key (FKPerson, FKRole)
);
create table Movie
(
  PK            int unsigned auto_increment primary key,
  FKName        int unsigned not null,
  FKDescription int unsigned,
  Airing        date,
  Length        smallint unsigned,
  AverageScore  decimal(4, 2),
  FKStatus      int unsigned,
  ImageSource   varchar(255),
  Added         date         not null DEFAULT (CURRENT_DATE),
  foreign key (FKName) references Translation (PK),
  foreign key (FKDescription) references Translation (PK),
  foreign key (FKStatus) references `Status` (PK)
);
create table MovieXGenre
(
  FKMovie int unsigned not null,
  FKGenre int unsigned not null,
  foreign key (FKMovie) references Movie (PK),
  foreign key (FKGenre) references Genre (PK),
  primary key (FKMovie, FKGenre)
);
create table MovieXTheme
(
  FKMovie int unsigned not null,
  FKTheme int unsigned not null,
  foreign key (FKMovie) references Movie (PK),
  foreign key (FKTheme) references Theme (PK),
  primary key (FKMovie, FKTheme)
);
create table MovieXCreator
(
  FKMovie  int unsigned not null,
  FKPerson int unsigned not null,
  FKRole   int unsigned not null,
  foreign key (FKMovie) references Movie (PK),
  foreign key (FKPerson) references Person (PK),
  foreign key (FKRole) references `Role` (PK),
  primary key (FKMovie, FKPerson, FKRole)
);
--
create table Album
(
  PK            int unsigned auto_increment primary key,
  `Name`        varchar(100) not null,
  FKDescription int unsigned,
  `Release`     date,
  Songs         tinyint unsigned,
  AverageScore  decimal(4, 2),
  ImageSource   varchar(255),
  Added         date         not null DEFAULT (CURRENT_DATE),
  foreign key (FKDescription) references Translation (PK)
);
create table AlbumXCreator
(
  FKAlbum  int unsigned not null,
  FKPerson int unsigned not null,
  FKRole   int unsigned not null,
  foreign key (FKAlbum) references Album (PK),
  foreign key (FKPerson) references Person (PK),
  foreign key (FKRole) references `Role` (PK),
  primary key (FKAlbum, FKPerson, FKRole)
);
create table Song
(
  PK            int unsigned auto_increment primary key,
  `Name`        varchar(100) not null,
  FKDescription int unsigned,
  FKAlbum       int unsigned not null,
  `Release`     date,
  Songs         tinyint unsigned,
  AverageScore  decimal(4, 2),
  ImageSource   varchar(255),
  Added         date         not null DEFAULT (CURRENT_DATE),
  foreign key (FKDescription) references Translation (PK),
  foreign key (FKAlbum) references Album (PK)
);
create table SongXGenre
(
  FKSong  int unsigned not null,
  FKGenre int unsigned not null,
  foreign key (FKSong) references Song (PK),
  foreign key (FKGenre) references Genre (PK),
  primary key (FKSong, FKGenre)
);
create table SongXCreator
(
  FKSong   int unsigned not null,
  FKPerson int unsigned not null,
  FKRole   int unsigned not null,
  foreign key (FKSong) references Song (PK),
  foreign key (FKPerson) references Person (PK),
  foreign key (FKRole) references `Role` (PK),
  primary key (FKSong, FKPerson, FKRole)
);
create table Manga
(
  PK            int unsigned auto_increment primary key,
  FKName        int unsigned not null,
  FKDescription int unsigned,
  PublishStart  date,
  PublishEnd    date,
  Volumes       smallint unsigned,
  Chapters      smallint unsigned,
  AverageScore  decimal(4, 2),
  FKStatus      int unsigned,
  foreign key (FKName) references Translation (PK),
  foreign key (FKDescription) references Translation (PK),
  foreign key (FKStatus) references `Status` (PK),
  ImageSource   varchar(255),
  Added         date         not null DEFAULT (CURRENT_DATE)
);
create table MangaXGenre
(
  FKManga int unsigned not null,
  FKGenre int unsigned not null,
  foreign key (FKManga) references Manga (PK),
  foreign key (FKGenre) references Genre (PK),
  primary key (FKManga, FKGenre)
);
create table MangaXTheme
(
  FKManga int unsigned not null,
  FKTheme int unsigned not null,
  foreign key (FKManga) references Manga (PK),
  foreign key (FKTheme) references Theme (PK),
  primary key (FKManga, FKTheme)
);
create table MangaXCreator
(
  FKManga  int unsigned not null,
  FKPerson int unsigned not null,
  FKRole   int unsigned not null,
  foreign key (FKManga) references Manga (PK),
  foreign key (FKPerson) references Person (PK),
  foreign key (FKRole) references `Role` (PK),
  primary key (FKManga, FKPerson, FKRole)
);
create table Anime
(
  PK            int unsigned auto_increment primary key,
  FKName        int unsigned not null,
  FKDescription int unsigned,
  AiringStart   date,
  AiringEnd     date,
  Episodes      smallint unsigned,
  Seasons       smallint unsigned,
  AverageScore  decimal(4, 2),
  FKStatus      int unsigned,
  ImageSource   varchar(255),
  Added         date         not null DEFAULT (CURRENT_DATE),
  foreign key (FKName) references Translation (PK),
  foreign key (FKDescription) references Translation (PK),
  foreign key (FKStatus) references `Status` (PK)
);
create table AnimeSeason
(
  PK            int unsigned auto_increment primary key,
  FKAnime       int unsigned not null,
  FKTitle       int unsigned not null,
  FKDescription int unsigned,
  Episodes      smallint     not null,
  PublishStart  date,
  PublishEnd    date,
  AverageScore  tinyint,
  foreign key (FKAnime) references Anime (PK),
  foreign key (FKTitle) references Translation (PK),
  foreign key (FKDescription) references Translation (PK)
);
create table AnimeEpisode
(
  PK            int unsigned auto_increment primary key,
  FKAnime       int unsigned not null,
  FKAnimeSeason int unsigned,
  FKName        int unsigned not null,
  FKDescription int unsigned,
  Length        smallint     not null,
  AiringDate    date,
  AverageScore  tinyint,
  foreign key (FKAnime) references Anime (PK),
  foreign key (FKAnimeSeason) references AnimeSeason (PK),
  foreign key (FKName) references Translation (PK),
  foreign key (FKDescription) references Translation (PK)
);
create table AnimeXCharacter
(
  FKAnime     int unsigned not null,
  FKCharacter int unsigned not null,
  foreign key (FKAnime) references Anime (PK),
  foreign key (FKCharacter) references `Character` (PK),
  primary key (FKAnime, FKCharacter)
);
create table AnimeXGenre
(
  FKAnime int unsigned not null,
  FKGenre int unsigned not null,
  foreign key (FKAnime) references Anime (PK),
  foreign key (FKGenre) references Genre (PK),
  primary key (FKAnime, FKGenre)
);
create table AnimeXTheme
(
  FKAnime int unsigned not null,
  FKTheme int unsigned not null,
  foreign key (FKAnime) references Anime (PK),
  foreign key (FKTheme) references Theme (PK),
  primary key (FKAnime, FKTheme)
);
create table AnimeXCreator
(
  FKAnime  int unsigned not null,
  FKPerson int unsigned not null,
  FKRole   int unsigned not null,
  foreign key (FKAnime) references Anime (PK),
  foreign key (FKPerson) references Person (PK),
  foreign key (FKRole) references `Role` (PK),
  primary key (FKAnime, FKPerson, FKRole)
);
create table Comic
(
  PK            int unsigned auto_increment primary key,
  FKName        int unsigned not null,
  FKDescription int unsigned,
  PublishStart  date,
  PublishEnd    date,
  Volumes       smallint unsigned,
  Chapters      smallint unsigned,
  AverageScore  decimal(4, 2),
  FKStatus      int unsigned,
  ImageSource   varchar(255),
  Added         date         not null DEFAULT (CURRENT_DATE),
  foreign key (FKName) references Translation (PK),
  foreign key (FKDescription) references Translation (PK),
  foreign key (FKStatus) references `Status` (PK)
);
create table ComicVolume
(
  PK            int unsigned auto_increment primary key,
  FKComic       int unsigned not null,
  FKTitle       int unsigned not null,
  FKDescription int unsigned,
  Pages         smallint     not null,
  PublishDate   date,
  AverageScore  tinyint,
  foreign key (FKComic) references Comic (PK),
  foreign key (FKTitle) references Translation (PK),
  foreign key (FKDescription) references Translation (PK)
);
create table ComicChapter
(
  PK            int unsigned auto_increment primary key,
  FKComic       int unsigned not null,
  FKComicVolume int unsigned,
  FKTitle       int unsigned not null,
  FKDescription int unsigned,
  Pages         smallint     not null,
  PublishDate   date,
  AverageScore  tinyint,
  foreign key (FKComic) references Comic (PK),
  foreign key (FKComicVolume) references ComicVolume (PK),
  foreign key (FKTitle) references Translation (PK),
  foreign key (FKDescription) references Translation (PK)
);

create table ComicXCharacter
(
  FKComic     int unsigned not null,
  FKCharacter int unsigned not null,
  foreign key (FKComic) references Comic (PK),
  foreign key (FKCharacter) references `Character` (PK),
  primary key (FKComic, FKCharacter)
);
create table ComicXGenre
(
  FKComic int unsigned not null,
  FKGenre int unsigned not null,
  foreign key (FKComic) references Comic (PK),
  foreign key (FKGenre) references Genre (PK),
  primary key (FKComic, FKGenre)
);
create table ComicXTheme
(
  FKComic int unsigned not null,
  FKTheme int unsigned not null,
  foreign key (FKComic) references Comic (PK),
  foreign key (FKTheme) references Theme (PK),
  primary key (FKComic, FkTheme)
);
create table ComicXCreator
(
  FKComic  int unsigned not null,
  FKPerson int unsigned not null,
  FKRole   int unsigned not null,
  foreign key (FKComic) references Comic (PK),
  foreign key (FKPerson) references Person (PK),
  foreign key (FKRole) references `Role` (PK),
  primary key (FKComic, FKPerson, FKRole)
);
create table Cartoon
(
  PK            int unsigned auto_increment primary key,
  FKName        int unsigned not null,
  FKDescription int unsigned,
  AiringStart   date,
  AiringEnd     date,
  AverageScore  decimal(4, 2),
  FKStatus      int unsigned,
  ImageSource   varchar(255),
  Added         date         not null DEFAULT (CURRENT_DATE),
  foreign key (FKName) references Translation (PK),
  foreign key (FKDescription) references Translation (PK),
  foreign key (FKStatus) references `Status` (PK)
);
create table CartoonSeason
(
  PK            int unsigned auto_increment primary key,
  FKCartoon     int unsigned not null,
  FKTitle       int unsigned not null,
  FKDescription int unsigned,
  Episodes      smallint,
  AiringStart   date,
  AiringEnd     date,
  AverageScore  tinyint,
  foreign key (FKCartoon) references Cartoon (PK),
  foreign key (FKTitle) references Translation (PK),
  foreign key (FKDescription) references Translation (PK)
);
create table CartoonEpisode
(
  PK              int unsigned auto_increment primary key,
  FKCartoon       int unsigned not null,
  FKCartoonSeason int unsigned,
  FKTitle         int unsigned not null,
  FKDescription   int unsigned,
  Length          smallint,
  AiringDate      date,
  AverageScore    tinyint,
  foreign key (FKCartoon) references Cartoon (PK),
  foreign key (FKCartoonSeason) references CartoonSeason (PK),
  foreign key (FKTitle) references Translation (PK),
  foreign key (FKDescription) references Translation (PK)
);
create table CartoonXCharacter
(
  FKCartoon   int unsigned not null,
  FKCharacter int unsigned not null,
  foreign key (FKCartoon) references Cartoon (PK),
  foreign key (FKCharacter) references `Character` (PK),
  primary key (FKCartoon, FKCharacter)
);
create table CartoonXGenre
(
  FKCartoon int unsigned not null,
  FKGenre   int unsigned not null,
  foreign key (FKCartoon) references Cartoon (PK),
  foreign key (FKGenre) references Genre (PK),
  primary key (FKCartoon, FKGenre)
);
create table CartoonXTheme
(
  FKCartoon int unsigned not null,
  FKTheme   int unsigned not null,
  foreign key (FKCartoon) references Cartoon (PK),
  foreign key (FKTheme) references Theme (PK),
  primary key (FKCartoon, FKTheme)
);
create table CartoonXCreator
(
  FKCartoon int unsigned not null,
  FKRole    int unsigned not null,
  FKPerson  int unsigned not null,
  foreign key (FKCartoon) references Cartoon (PK),
  foreign key (FKRole) references `Role` (PK),
  foreign key (FKPerson) references Person (PK),
  primary key (FKCartoon, FKRole, FKPerson)
);
create table Book
(
  PK            int unsigned auto_increment primary key,
  FKName        int unsigned not null,
  FKDescription int unsigned,
  Chapters      tinyint unsigned,
  Pages         smallint unsigned,
  Words         smallint unsigned,
  PublishDate   date,
  AverageScore  decimal(4, 2),
  FKStatus      int unsigned,
  ImageSource   varchar(255),
  Added         date         not null DEFAULT (CURRENT_DATE),
  foreign key (FKName) references Translation (PK),
  foreign key (FKDescription) references Translation (PK),
  foreign key (FKStatus) references `Status` (PK)
);
create table BookXCharacter
(
  FKBook      int unsigned not null,
  FKCharacter int unsigned not null,
  foreign key (FKBook) references Book (PK),
  foreign key (FKCharacter) references `Character` (PK),
  primary key (FKBook, FKCharacter)
);
create table BookXGenre
(
  FKBook  int unsigned not null,
  FKGenre int unsigned not null,
  foreign key (FKBook) references Book (PK),
  foreign key (FKGenre) references Genre (PK),
  primary key (FKBook, FKGenre)
);
create table BookXTheme
(
  FKBook  int unsigned not null,
  FKTheme int unsigned not null,
  foreign key (FKBook) references Book (PK),
  foreign key (FKTheme) references Theme (PK),
  primary key (FKBook, FKTheme)
);
create table BookXCreator
(
  FKBook   int unsigned not null,
  FKRole   int unsigned not null,
  FKPerson int unsigned not null,
  foreign key (FKBook) references Book (PK),
  foreign key (FKRole) references `Role` (PK),
  foreign key (FKPerson) references Person (PK),
  primary key (FKBook, FKRole, FKPerson)
);
create table TVShow
(
  PK            int unsigned auto_increment primary key,
  FKName        int unsigned not null,
  FKDescription int unsigned,
  AiringStart   date,
  AiringEnd     date,
  AverageScore  decimal(4, 2),
  FKStatus      int unsigned,
  ImageSource   varchar(255),
  Added         date         not null DEFAULT (CURRENT_DATE),
  foreign key (FKName) references Translation (PK),
  foreign key (FKDescription) references Translation (PK),
  foreign key (FKStatus) references `Status` (PK)
);
create table TVShowSeason
(
  PK            int unsigned auto_increment primary key,
  FKTVShow      int unsigned not null,
  FKTitle       int unsigned not null,
  FKDescription int unsigned,
  Episodes      smallint,
  AiringStart   date,
  AiringEnd     date,
  AverageScore  tinyint,
  foreign key (FKTVShow) references TVShow (PK),
  foreign key (FKTitle) references Translation (PK),
  foreign key (FKDescription) references Translation (PK)
);
create table TVShowEpisode
(
  PK             int unsigned auto_increment primary key,
  FKTVShow       int unsigned not null,
  FKTVShowSeason int unsigned,
  FKTitle        int unsigned not null,
  FKDescription  int unsigned,
  Length         smallint,
  AiringDate     date,
  AverageScore   tinyint,
  foreign key (FKTVShow) references TVShow (PK),
  foreign key (FKTVShowSeason) references TVShowSeason (PK),
  foreign key (FKTitle) references Translation (PK),
  foreign key (FKDescription) references Translation (PK)
);
create table TVShowXCharacter
(
  FKTVShow    int unsigned not null,
  FKCharacter int unsigned not null,
  foreign key (FKTVShow) references TVShow (PK),
  foreign key (FKCharacter) references `Character` (PK),
  primary key (FKTVShow, FKCharacter)
);
create table TVShowXGenre
(
  FKTVShow int unsigned not null,
  FKGenre  int unsigned not null,
  foreign key (FKTVShow) references TVShow (PK),
  foreign key (FKGenre) references Genre (PK),
  primary key (FKTVShow, FKGenre)
);
create table TVShowXTheme
(
  FKTVShow int unsigned not null,
  FKTheme  int unsigned not null,
  foreign key (FKTVShow) references TVShow (PK),
  foreign key (FKTheme) references Theme (PK),
  primary key (FKTVShow, FKTheme)
);
create table TVShowXCreator
(
  FKTVShow int unsigned not null,
  FKRole   int unsigned not null,
  FKPerson int unsigned not null,
  foreign key (FKTVShow) references TVShow (PK),
  foreign key (FKRole) references `Role` (PK),
  foreign key (FKPerson) references Person (PK),
  primary key (FKTVShow, FKRole, FKPerson)
);
create table Game
(
  PK            int unsigned auto_increment primary key,
  FKName        int unsigned not null,
  FKDescription int unsigned,
  Published     date,
  AverageScore  decimal(4, 2),
  ImageSource   varchar(255),
  Added         date         not null DEFAULT (CURRENT_DATE),
  foreign key (FKName) references Translation (PK),
  foreign key (FKDescription) references Translation (PK)
);
create table GameXCharacter
(
  FKGame      int unsigned not null,
  FKCharacter int unsigned not null,
  foreign key (FKGame) references Game (PK),
  foreign key (FKCharacter) references `Character` (PK),
  primary key (FKGame, FKCharacter)
);
create table GameXGenre
(
  FKGame  int unsigned not null,
  FKGenre int unsigned not null,
  foreign key (FKGame) references Game (PK),
  foreign key (FKGenre) references Genre (PK),
  primary key (FKGame, FKGenre)
);
create table GameXTheme
(
  FKGame  int unsigned not null,
  FKTheme int unsigned not null,
  foreign key (FKGame) references Game (PK),
  foreign key (FKTheme) references Theme (PK),
  primary key (FKGame, FKTheme)
);
create table GameXCreator
(
  FKGame   int unsigned not null,
  FKRole   int unsigned not null,
  FKPerson int unsigned not null,
  foreign key (FKGame) references Game (PK),
  foreign key (FKRole) references `Role` (PK),
  foreign key (FKPerson) references Person (PK),
  primary key (FKGame, FKRole, FKPerson)
);
create table `User`
(
  PK            int unsigned auto_increment primary key,
  FKPerson      int unsigned,
  `Name`        varchar(50)  not null,
  Joined        date         not null DEFAULT (CURRENT_DATE),
  `Description` varchar(500),
  ImageSource   varchar(255),
  foreign key (FKPerson) references Person (PK)
);
create table Average
(
  PK             int unsigned auto_increment primary key,
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
  foreign key (FKUser) references `User` (PK)
);
create table UserXAlbum
(
  FKUser   int unsigned not null,
  FKAlbum  int unsigned not null,
  Favorite bit,
  Score    tinyint unsigned,
  Review   varchar(255),
  Added    date         not null DEFAULT (CURRENT_DATE),
  foreign key (FKUser) references `User` (PK),
  foreign key (FKAlbum) references Album (PK),
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
  foreign key (FKUser) references `User` (PK),
  foreign key (FKSong) references Song (PK),
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
  foreign key (FKUser) references `User` (PK),
  foreign key (FKManga) references Manga (PK),
  foreign key (FKUserStatus) references UserStatus (PK),
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
  foreign key (FKUser) references `User` (PK),
  foreign key (FKComic) references Comic (PK),
  foreign key (FKUserStatus) references UserStatus (PK),
  primary key (FKUser, FKComic)
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
  foreign key (FKUser) references `User` (PK),
  foreign key (FKTVShow) references TVShow (PK),
  foreign key (FKUserStatus) references UserStatus (PK),
  primary key (FKUser, FKTVShow)
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
  foreign key (FKUser) references `User` (PK),
  foreign key (FKAnime) references Anime (PK),
  foreign key (FKUserStatus) references UserStatus (PK),
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
  foreign key (FKUser) references `User` (PK),
  foreign key (FKGame) references Game (PK),
  foreign key (FKUserStatus) references UserStatus (PK),
  primary key (FKUser, FKGame)
);
create table Friendship
(
  PK           int unsigned auto_increment primary key,
  FKUser       int unsigned not null,
  FKSecondUser int unsigned not null,
  Added        date         not null DEFAULT (CURRENT_DATE),
  foreign key (FKUser) references `User` (PK),
  foreign key (FKSecondUser) references `User` (PK)
);
create table `Account`
(
  FKUser     int unsigned not null primary key,
  EMail      varchar(255) not null,
  `Password` char(48)     not null,
  CONSTRAINT UN_EMAIL UNIQUE (EMail),
  foreign key (FKUser) references `User` (PK)
);
commit;
