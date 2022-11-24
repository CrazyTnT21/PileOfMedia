create table TTranslation(
PK int unsigned not null auto_increment primary key,
English varchar(500),
German varchar(500),
Spanish varchar(500),
Japanese nvarchar(500)
);
-- TPublish
create table TPublish(
PK int unsigned not null auto_increment primary key,
FKPublish int unsigned not null, foreign key (FKPublish) references TTranslation(PK)
);
-- TRelation
create table TRelation(
PK int unsigned not null auto_increment primary key,
FKRelation int unsigned not null, foreign key (FKRelation) references TTranslation(PK)
);
-- insert into TRelation(Relation) values("Unkown"),("Prequel"),("Sequel"),("Adaptation");
-- TGenre
create table TGenre(
PK int unsigned not null auto_increment primary key,
FKGenre int unsigned not null, foreign key (FKGenre) references TTranslation(PK)
);
-- insert into TGenre(Genre) values("Action"),("Romance"),("Horror");
-- TStatus
create table TStatus(
PK int unsigned not null auto_increment primary key,
FKStatus int unsigned not null, foreign key (FKStatus) references TTranslation(PK)
);
-- insert into TStatus(Status) values("Unkown"),("Not started"),("Running"),("Publishing"),("Finished"),("Hiatus");
-- TUserStatus
create table TUserStatus(
PK int unsigned not null auto_increment primary key,
FKUserStatus int unsigned not null, foreign key (FKUserStatus) references TTranslation(PK)
);
-- insert into TUserStatus(UserStatus) values("Not started"),("Reading"),("Watching"),("Finished"),("Paused");
-- TTheme
create table TTheme(
PK int unsigned not null auto_increment primary key,
FKThene int unsigned not null, foreign key (FKThene) references TTranslation(PK)
);
-- insert into TTheme(Theme) values("Psychological"),("Gore"),("Mythology"),("Military");
-- TRole
create table TRole(
PK int unsigned not null auto_increment primary key,
FKRole int unsigned not null, foreign key (FKRole) references TTranslation(PK)
);
-- insert into TRole(`Role`) values("Unkown"),("User"),("Director"),("Artist"),("Writer"),("Voice Actor"),("Sound designer"),("Producer");

-- TCharacter
create table TCharacter(
PK int unsigned not null auto_increment primary key,
FKName int unsigned not null, foreign key (FKName) references TTranslation(PK),
FKFirstName int unsigned not null, foreign key (FKFirstName) references TTranslation(PK),
FKLastName int unsigned not null, foreign key (FKLastName) references TTranslation(PK),
FKDescription int unsigned not null, foreign key (FKDescription) references TTranslation(PK),
Birthday date,
Height tinyint unsigned,
ImageSource varchar(255)
);
-- TPerson
create table TPerson(
PK int unsigned not null auto_increment primary key,
`Name` varchar(50) not null,
FirstName varchar(50),
LastName varchar(50),
Description varchar(500),
Birthday date,
Height tinyint unsigned,
ImageSource varchar(255)
);
create table TPersonXTRole(
PK int unsigned not null auto_increment primary key,
FKPerson int unsigned not null, foreign key (FKPerson) references TPerson(PK),
FKRole int unsigned not null, foreign key (FKRole) references TRole(PK)
);
-- TMovie
create table TMovie(
PK int unsigned not null auto_increment primary key,
`Name` varchar(50) not null,
`Description` varchar(500),
Synopsis varchar(500),
Airing date,
Length smallint unsigned,
AverageScore decimal(4,2),
FKStatus int unsigned, foreign key (FKStatus) references TStatus(PK),
ImageSource varchar(255)
);
create table TMovieXGenre(
PK int unsigned not null auto_increment primary key,
FKMovie int unsigned not null, foreign key (FKMovie) references TMovie(PK),
FKGenre int unsigned not null, foreign key (FKGenre) references TGenre(PK)
);
create table TMovieXTheme(
PK int unsigned not null auto_increment primary key,
FKMovie int unsigned not null, foreign key (FKMovie) references TMovie(PK),
FKTheme int unsigned not null, foreign key (FKTheme) references TTheme(PK)
);
create table TMovieXCreator(
PK int unsigned not null auto_increment primary key,
FKRole int unsigned not null, foreign key (FKRole) references TRole(PK),
FKMovie int unsigned not null, foreign key (FKMovie) references TMovie(PK),
FKPerson int unsigned not null, foreign key (FKPerson) references Tperson(PK)
);
-- TManga
create table TManga(
PK int unsigned not null auto_increment primary key,
`Name` varchar(50) not null,
`Description` varchar(500),
Synopsis varchar(500),
PublishStart date,
PublishEnd date,
Volumes smallint unsigned,
Chapters smallint unsigned,
AverageScore decimal(4,2),
FKStatus int unsigned, foreign key (FKStatus) references TStatus(PK),
ImageSource varchar(255),
FKPublish int unsigned, foreign key (FKPublish) references TPublish(PK)
);
create table TMangaXGenre(
PK int unsigned not null auto_increment primary key,
FKManga int unsigned not null, foreign key (FKManga) references TManga(PK),
FKGenre int unsigned not null, foreign key (FKGenre) references TGenre(PK)
);
create table TMangaXTheme(
PK int unsigned not null auto_increment primary key,
FKManga int unsigned not null, foreign key (FKManga) references TManga(PK),
FKTheme int unsigned not null, foreign key (FKTheme) references TTheme(PK)
);
create table TMangaXCreator(
PK int unsigned not null auto_increment primary key,
FKRole int unsigned not null, foreign key (FKRole) references TRole(PK),
FKManga int unsigned not null, foreign key (FKManga) references TManga(PK),
FKPerson int unsigned not null, foreign key (FKPerson) references Tperson(PK)
);
-- TAnime
create table TAnime(
PK int unsigned not null auto_increment primary key,
`Name` varchar(50) not null,
`Description` varchar(500),
Synopsis varchar(500),
AiringStart date,
AiringEnd date,
Episodes smallint unsigned,
Seasons smallint unsigned,
AverageScore decimal(4,2),
FKStatus int unsigned, foreign key (FKStatus) references TStatus(PK),
ImageSource varchar(255)
);
create table TAnimeSeason(
PK int unsigned not null auto_increment primary key,
FKAnime int unsigned not null, foreign key (FKAnime) references TAnime(PK),
Title varchar(50) not null,
`Description` varchar(500),
Synopsis varchar(500),
Episodes smallint not null,
PublishStart date,
PublishEnd date,
AverageScore tinyint
);
create table TAnimeEpisode(
PK int unsigned not null auto_increment primary key,
FKAnime int unsigned not null, foreign key (FKAnime) references TAnime(PK),
FKAnimeSeason int unsigned, foreign key (FKAnimeSeason) references TAnimeSeason(PK),
Title varchar(50) not null,
Synopsis varchar(500),
Length smallint not null,
AiringDate date,
AverageScore tinyint
);
create table TAnimeXCharacter(
PK int unsigned not null auto_increment primary key,
FKAnime int unsigned not null, foreign key (FKAnime) references TAnime(PK),
FKCharacter int unsigned not null, foreign key (FKCharacter) references TCharacter(PK)
);
create table TAnimeXGenre(
PK int unsigned not null auto_increment primary key,
FKAnime int unsigned not null, foreign key (FKAnime) references TAnime(PK),
FKGenre int unsigned not null, foreign key (FKGenre) references TGenre(PK)
);
create table TAnimeXTheme(
PK int unsigned not null auto_increment primary key,
FKAnime int unsigned not null, foreign key (FKAnime) references TAnime(PK),
FKTheme int unsigned not null, foreign key (FKTheme) references TTheme(PK)
);
create table TAnimeXCreator(
PK int unsigned not null auto_increment primary key,
FKRole int unsigned not null, foreign key (FKRole) references TRole(PK),
FKAnime int unsigned not null, foreign key (FKAnime) references TAnime(PK),
FKPerson int unsigned not null, foreign key (FKPerson) references Tperson(PK)
);
-- TComic
create table TComic(
PK int unsigned not null auto_increment primary key,
FKName int unsigned not null, foreign key (FKName) references TTranslation(PK),
FKDescription int unsigned, foreign key (FKDescription) references TTranslation(PK),
FKSynopsis int unsigned, foreign key (FKSynopsis) references TTranslation(PK),
PublishStart date,
PublishEnd date,
Volumes smallint unsigned,
Chapters smallint unsigned,
AverageScore decimal(4,2),
FKStatus int unsigned, foreign key (FKStatus) references TStatus(PK),
ImageSource varchar(255)
);
create table TComicVolume(
PK int unsigned not null auto_increment primary key,
FKComic int unsigned not null, foreign key (FKComic) references TComic(PK),
Title varchar(50) not null,
`Description` varchar(500),
Synopsis varchar(500),
Pages smallint not null,
PublishDate date,
AverageScore tinyint
);
create table TComicChapter(
PK int unsigned not null auto_increment primary key,
FKComic int unsigned not null, foreign key (FKComic) references TComic(PK),
FKComicVolume int unsigned, foreign key (FKComicVolume) references TComicVolume(PK),
Title varchar(50) not null,
Synopsis varchar(500),
Pages smallint not null,
PublishDate date,
AverageScore tinyint
);

create table TComicXCharacter(
PK int unsigned not null auto_increment primary key,
FKComic int unsigned not null, foreign key (FKComic) references TComic(PK),
FKCharacter int unsigned not null, foreign key (FKCharacter) references TCharacter(PK)
);
create table TComicXGenre(
PK int unsigned not null auto_increment primary key,
FKComic int unsigned not null, foreign key (FKComic) references TComic(PK),
FKGenre int unsigned not null, foreign key (FKGenre) references TGenre(PK)
);
create table TComicXTheme(
PK int unsigned not null auto_increment primary key,
FKComic int unsigned not null, foreign key (FKComic) references TComic(PK),
FKTheme int unsigned not null, foreign key (FKTheme) references TTheme(PK)
);
create table TComicXCreator(
PK int unsigned not null auto_increment primary key,
FKRole int unsigned not null, foreign key (FKRole) references TRole(PK),
FKComic int unsigned not null, foreign key (FKComic) references TComic(PK),
FKPerson int unsigned not null, foreign key (FKPerson) references Tperson(PK)
);
-- TCartoon
create table TCartoon(
PK int unsigned not null auto_increment primary key,
`Name` varchar(50) not null,
`Description` varchar(500),
Synopsis varchar(500),
AiringStart date,
AiringEnd date,
AverageScore decimal(4,2),
FKStatus int unsigned, foreign key (FKStatus) references TStatus(PK),
ImageSource varchar(255),
FKPublish int unsigned, foreign key (FKPublish) references TPublish(PK)
);
create table TCartoonSeason(
PK int unsigned not null auto_increment primary key,
FKCartoon int unsigned not null, foreign key (FKCartoon) references TCartoon(PK),
Title varchar(50) not null,
`Description` varchar(500),
Synopsis varchar(500),
Episodes smallint,
AiringStart date,
AiringEnd date,
AverageScore tinyint
);
create table TCartoonEpisode(
PK int unsigned not null auto_increment primary key,
FKCartoon int unsigned not null, foreign key (FKCartoon) references TCartoon(PK),
FKCartoonSeason int unsigned, foreign key (FKCartoonSeason) references TCartoonSeason(PK),
Title varchar(50) not null,
Synopsis varchar(500),
Length smallint,
AiringDate date,
AverageScore tinyint
);
create table TCartoonXCharacter(
PK int unsigned not null auto_increment primary key,
FKCartoon int unsigned not null, foreign key (FKCartoon) references TCartoon(PK),
FKCharacter int unsigned not null, foreign key (FKCharacter) references TCharacter(PK)
);
create table TCartoonXGenre(
PK int unsigned not null auto_increment primary key,
FKCartoon int unsigned not null, foreign key (FKCartoon) references TCartoon(PK),
FKGenre int unsigned not null, foreign key (FKGenre) references TGenre(PK)
);
create table TCartoonXTheme(
PK int unsigned not null auto_increment primary key,
FKCartoon int unsigned not null, foreign key (FKCartoon) references TCartoon(PK),
FKTheme int unsigned not null, foreign key (FKTheme) references TTheme(PK)
);
create table TCartoonXCreator(
PK int unsigned not null auto_increment primary key,
FKRole int unsigned not null, foreign key (FKRole) references TRole(PK),
FKCartoon int unsigned not null, foreign key (FKCartoon) references TCartoon(PK),
FKPerson int unsigned not null, foreign key (FKPerson) references Tperson(PK)
);
-- TBook
create table TBook(
PK int unsigned not null auto_increment primary key,
`Name` varchar(50) not null,
`Description` varchar(500),
Synopsis varchar(500),
Pages smallint unsigned,
Words smallint unsigned,
PublishDate date,
AverageScore decimal(4,2),
FKStatus int unsigned, foreign key (FKStatus) references TStatus(PK),
ImageSource varchar(255)
);
create table TBookXCharacter(
PK int unsigned not null auto_increment primary key,
FKBook int unsigned not null, foreign key (FKBook) references TBook(PK),
FKCharacter int unsigned not null, foreign key (FKCharacter) references TCharacter(PK)
);
create table TBookXGenre(
PK int unsigned not null auto_increment primary key,
FKBook int unsigned not null, foreign key (FKBook) references TBook(PK),
FKGenre int unsigned not null, foreign key (FKGenre) references TGenre(PK)
);
create table TBookXTheme(
PK int unsigned not null auto_increment primary key,
FKBook int unsigned not null, foreign key (FKBook) references TBook(PK),
FKTheme int unsigned not null, foreign key (FKTheme) references TTheme(PK)
);
create table TBookXCreator(
PK int unsigned not null auto_increment primary key,
FKRole int unsigned not null, foreign key (FKRole) references TRole(PK),
FKBook int unsigned not null, foreign key (FKBook) references TBook(PK),
FKPerson int unsigned not null, foreign key (FKPerson) references Tperson(PK)
);
-- TTVShow
create table TTVShow(
PK int unsigned not null auto_increment primary key,
`Name` varchar(50) not null,
`Description` varchar(500),
Synopsis varchar(500),
AiringStart date,
AiringEnd date,
AverageScore decimal(4,2),
FKStatus int unsigned, foreign key (FKStatus) references TStatus(PK),
ImageSource varchar(255),
FKPublish int unsigned, foreign key (FKPublish) references TPublish(PK)
);
create table TTVShowSeason(
PK int unsigned not null auto_increment primary key,
FKTVShow int unsigned not null, foreign key (FKTVShow) references TTVShow(PK),
Title varchar(50) not null,
`Description` varchar(500),
Synopsis varchar(500),
Episodes smallint,
AiringStart date,
AiringEnd date,
AverageScore tinyint
);
create table TTVShowEpisode(
PK int unsigned not null auto_increment primary key,
FKTVShow int unsigned not null, foreign key (FKTVShow) references TTVShow(PK),
FKTVShowSeason int unsigned, foreign key (FKTVShowSeason) references TTVShowSeason(PK),
Title varchar(50) not null,
Synopsis varchar(500),
Length smallint,
AiringDate date,
AverageScore tinyint
);
create table TTVShowXCharacter(
PK int unsigned not null auto_increment primary key,
FKTVShow int unsigned not null, foreign key (FKTVShow) references TTVShow(PK),
FKCharacter int unsigned not null, foreign key (FKCharacter) references TCharacter(PK)
);
create table TTVShowXGenre(
PK int unsigned not null auto_increment primary key,
FKTVShow int unsigned not null, foreign key (FKTVShow) references TTVShow(PK),
FKGenre int unsigned not null, foreign key (FKGenre) references TGenre(PK)
);
create table TTVShowXTheme(
PK int unsigned not null auto_increment primary key,
FKTVShow int unsigned not null, foreign key (FKTVShow) references TTVShow(PK),
FKTheme int unsigned not null, foreign key (FKTheme) references TTheme(PK)
);
create table TTVShowXCreator(
PK int unsigned not null auto_increment primary key,
FKRole int unsigned not null, foreign key (FKRole) references TRole(PK),
FKTVShow int unsigned not null, foreign key (FKTVShow) references TTVShow(PK),
FKPerson int unsigned not null, foreign key (FKPerson) references TPerson(PK)
);
-- TGame
create table TGame(
PK int unsigned not null auto_increment primary key,
`Name` varchar(50) not null,
`Description` varchar(500),
Synopsis varchar(500),
Published date,
AverageScore decimal(4,2),
FKStatus int unsigned, foreign key (FKStatus) references TStatus(PK),
ImageSource varchar(255)
);
create table TGameXCharacter(
PK int unsigned not null auto_increment primary key,
FKGame int unsigned not null, foreign key (FKGame) references TGame(PK),
FKCharacter int unsigned not null, foreign key (FKCharacter) references TCharacter(PK)
);
create table TGameXGenre(
PK int unsigned not null auto_increment primary key,
FKGame int unsigned not null, foreign key (FKGame) references TGame(PK),
FKGenre int unsigned not null, foreign key (FKGenre) references TGenre(PK)
);
create table TGameXTheme(
PK int unsigned not null auto_increment primary key,
FKGame int unsigned not null, foreign key (FKGame) references TGame(PK),
FKTheme int unsigned not null, foreign key (FKTheme) references TTheme(PK)
);
create table TGameXCreator(
PK int unsigned not null auto_increment primary key,
FKRole int unsigned not null, foreign key (FKRole) references TRole(PK),
FKGame int unsigned not null, foreign key (FKGame) references TGame(PK),
FKPerson int unsigned not null, foreign key (FKPerson) references Tperson(PK)
);
-- TUser
create table TUser(
PK int unsigned not null auto_increment primary key,
FKPerson int unsigned, foreign key (FKPerson) references TPerson(PK),
`Name` varchar(50) not null,
Joined date not null,
`Description` varchar(500),
ImageSource varchar(255),
MangaAverage decimal(4,2),
ComicAverage decimal(4,2),
TVShowAverage decimal(4,2),
MovieAverage decimal(4,2),
AnimeAverage decimal(4,2),
BookAverage decimal(4,2),
CartoonAverage decimal(4,2),
GameAverage decimal(4,2)
);
create table TUserXManga(
PK int unsigned not null auto_increment primary key,
FKUser int unsigned not null, foreign key (FKUser) references TUser(PK),
FKManga int unsigned not null, foreign key (FKManga) references TManga(PK),
FKUserStatus int unsigned not null, foreign key (FKUserStatus) references TUserStatus(PK),
Favorite bit,
Score tinyint unsigned,
Review varchar(255),
StartDate date,
EndDate date,
Chapters smallint unsigned,
Added date not null
);
create table TUserXComic(
PK int unsigned not null auto_increment primary key,
FKUser int unsigned not null, foreign key (FKUser) references TUser(PK),
FKComic int unsigned not null, foreign key (FKComic) references TComic(PK),
FKUserStatus int unsigned not null, foreign key (FKUserStatus) references TUserStatus(PK),
Favourite bit,
Score tinyint unsigned,
Review varchar(255),
StartDate date,
FinishedDate date,
Chapters smallint unsigned,
Added date not null
);
create table TUserXTVShow(
PK int unsigned not null auto_increment primary key,
FKUser int unsigned not null, foreign key (FKUser) references TUser(PK),
FKTVShow int unsigned not null, foreign key (FKTVShow) references TTVShow(PK),
FKUserStatus int unsigned not null, foreign key (FKUserStatus) references TUserStatus(PK),
Favourite bit,
Score tinyint unsigned,
Review varchar(255),
StartDate date,
FinishedDate date,
Episodes smallint unsigned,
Added date not null
);
create table TFriend(
PK int unsigned not null auto_increment primary key,
FKUser int unsigned not null, foreign key (FKUser) references TUser(PK),
FKSecondUser int unsigned not null, foreign key (FKSecondUser) references TUser(PK),
Added date not null
);
-- other
create table TLog(
PK int unsigned not null auto_increment primary key,
Log varchar(500) not null,
`Date` datetime not null
);
create table TUserXRight(
PK int unsigned not null auto_increment primary key,
FKUser int unsigned not null, foreign key (FKUser) references TUser(PK),
FKRight int unsigned not null, foreign key (FKRight) references TRight(PK)
);
create table TRight(
PK int unsigned not null auto_increment primary key,
FKArea int unsigned not null, foreign key (FKArea) references TArea(PK),
`Add` bit,
Edit bit,
Delte bit
);
create table TArea(
PK int unsigned not null auto_increment primary key,
Area varchar(50) not null,
);
create table TError(
PK int unsigned not null auto_increment primary key,
`Error` varchar(500) not null,
`Date` datetime not null
);