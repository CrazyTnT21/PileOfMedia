/*
drop database if exists collectiondb with (FORCE);
create database collectiondb with ENCODING 'UTF8';
*/
DO
$$
    DECLARE
        nintendoLogo  int;
        microsoftLogo int;
        sonyLogo      int;
        SegaLogo      int;
        atariLogo     int;
        valveLogo     int;
        cdProjektLogo int;
        itchioLogo    int;
        ubisoftLogo   int;
        eaLogo        int;
        epicgamesLogo int;

    BEGIN
        CREATE TYPE language AS ENUM ( 'EN','DE','ES','DA','NL','JA','KO');
        CREATE TYPE status AS ENUM ('NotStarted','Ongoing','Finished','Paused');
        CREATE TYPE userstatus AS ENUM ('NotStarted','Ongoing','Finished','Paused');
        CREATE TYPE imageextenstion as ENUM ('JPEG','JPG','PNG','GIF');
        create table Franchise
        (
            Id   int primary key generated always as identity,
            Name varchar(50) not null
        );
        create table Image
        (
            Id        int primary key generated always as identity,
            Uri       varchar(2047) not null,
            Width     smallint      not null,
            Height    smallint      not null,
            Extension imageextenstion
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
        -- Platforms
/*
 -- Nintendo
    Switch
    3DS
    ...
    Playstation
    XBox
    Steam
    Epic
    GOG
    Itch.io
    Other
    Id  Name  Logo  | FKPlatform => URI | Price (No API available)
 */
        create table Company
        (
            Id     int primary key generated always as identity,
            Name   varchar(100) not null,
            FKLogo int          not null references Image (Id)
        );

        insert into image(width, height, extension, uri)
        values (300, 300, 'PNG',
                'https://static-cdn.jtvnw.net/jtv_user_pictures/4074674c-ad77-412d-aafe-6d276b3cacda-profile_image-300x300.png')
        returning id into nintendoLogo;

--    TODO: Other logos
        INSERT INTO Company(Name, FKLogo) --https://static-cdn.jtvnw.net/jtv_user_pictures/4074674c-ad77-412d-aafe-6d276b3cacda-profile_image-300x300.png
        values ('Nintendo', nintendoLogo),
               ('Microsoft', nintendoLogo),
               ('Sony', nintendoLogo),
               ('Sega', nintendoLogo),
               ('Atari', nintendoLogo),
               ('Valve', nintendoLogo),
               ('CD Projekt', nintendoLogo),
               ('itch.io', nintendoLogo),
               ('Ubisoft', nintendoLogo),
               ('Electronics Arts', nintendoLogo),
               ('Epic Games', nintendoLogo);

        create table Platform
        (
            Id        int primary key generated always as identity,
            Name      varchar(50) not null,
            ShortName varchar(10),
            FKCompany int         not null references Company (Id),
            FKLogo    int         not null references Image (Id)
        );
        INSERT INTO Platform(Name, ShortName, FKCompany, FKLogo)
        values ('Switch', null, 1, nintendoLogo),
               ('3DS', null, 1, nintendoLogo),
               ('DS', null, 1, nintendoLogo),
               ('Wii U', null, 1, nintendoLogo),
               ('Wii', null, 1, nintendoLogo),
               ('Gamecube', null, 1, nintendoLogo),
               ('Gameboy Advance', 'GBA', 1, nintendoLogo),
               ('Gameboy Color', null, 1, nintendoLogo),
               ('Gameboy', null, 1, nintendoLogo),
               ('Game & Watch', null, 1, nintendoLogo),
               ('N64', null, 1, nintendoLogo),
               ('Super Nintendo Entertainment System', 'SNES', 1, nintendoLogo),
               ('Nintendo Entertainment System', 'NES', 1, nintendoLogo),
               ('Playstation', 'PS1', 3, nintendoLogo),
               ('Playstation 2', 'PS2', 3, nintendoLogo),
               ('Playstation 3', 'PS3', 3, nintendoLogo),
               ('Playstation 4', 'PS4', 3, nintendoLogo),
               ('Playstation 5', 'PS5', 3, nintendoLogo),
               ('XBox', null, 2, nintendoLogo),
               ('XBox 360', null, 2, nintendoLogo),
               ('XBox One', null, 2, nintendoLogo),
               ('XBox Series X', null, 2, nintendoLogo),
               ('Dreamcast', null, 4, nintendoLogo),
               ('Genesis', null, 4, nintendoLogo),
               ('Steam', null, 5, nintendoLogo),
               ('GOG.com', null, 6, nintendoLogo),
               ('itch.io', null, 7, nintendoLogo),
               ('Origin', null, 8, nintendoLogo),
               ('Battle.net', null, 9, nintendoLogo),
               ('Epic Games Store', null, 10, nintendoLogo);

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

        INSERT INTO Genre default
        values;
        INSERT INTO GenreTranslation(Name, FKTranslation, Language)
        values ('Action', 1, 'EN');
        INSERT INTO GenreTranslation(Name, FKTranslation, Language)
        values ('Action', 1, 'DE');

        INSERT INTO Genre default
        values;
        INSERT INTO GenreTranslation(Name, FKTranslation, Language)
        values ('Romance', 2, 'EN');
        INSERT INTO GenreTranslation(Name, FKTranslation, Language)
        values ('Romanze', 2, 'DE');

        INSERT INTO Genre default
        values;
        INSERT INTO GenreTranslation(Name, FKTranslation, Language)
        values ('Horror', 3, 'EN');
        INSERT INTO GenreTranslation(Name, FKTranslation, Language)
        values ('Horror', 3, 'DE');

        INSERT INTO Genre default
        values;
        INSERT INTO GenreTranslation(Name, FKTranslation, Language)
        values ('Comedy', 4, 'EN');
        INSERT INTO GenreTranslation(Name, FKTranslation, Language)
        values ('Komödie', 4, 'DE');

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
        INSERT INTO Theme default
        values;
        INSERT INTO ThemeTranslation(Name, FKTranslation, Language)
        values ('Psychological', 1, 'EN');
        INSERT INTO ThemeTranslation(Name, FKTranslation, Language)
        values ('Psychologisch', 1, 'DE');


        INSERT INTO Theme default
        values;
        INSERT INTO ThemeTranslation(Name, FKTranslation, Language)
        values ('Gore', 2, 'EN');
        INSERT INTO ThemeTranslation(Name, FKTranslation, Language)
        values ('Blutig', 2, 'DE');


        INSERT INTO Theme default
        values;
        INSERT INTO ThemeTranslation(Name, FKTranslation, Language)
        values ('Mythology', 3, 'EN');
        INSERT INTO ThemeTranslation(Name, FKTranslation, Language)
        values ('Mythologie', 3, 'DE');

        INSERT INTO Theme default
        values;
        INSERT INTO ThemeTranslation(Name, FKTranslation, Language)
        values ('Military', 4, 'EN');
        INSERT INTO ThemeTranslation(Name, FKTranslation, Language)
        values ('Militär', 4, 'DE');

        INSERT INTO Theme default
        values;
        INSERT INTO ThemeTranslation(Name, FKTranslation, Language)
        values ('Sinister', 5, 'EN');
        INSERT INTO ThemeTranslation(Name, FKTranslation, Language)
        values ('Finster', 5, 'DE');

        create table Role
        (
            Id int primary key generated always as identity
        );
        create table RoleTranslation
        (
            Name          varchar(50) not null,

            FKTranslation int         not null,
            Language      language    not null,
            foreign key (FKTranslation) references Role (Id),
            primary key (FKTranslation, Language)
        );

        INSERT INTO Role default
        values;
        INSERT INTO RoleTranslation(Name, FKTranslation, Language)
        values ('Director', 1, 'EN');
        INSERT INTO RoleTranslation(Name, FKTranslation, Language)
        values ('Regisseur', 1, 'DE');

        INSERT INTO Role default
        values;
        INSERT INTO RoleTranslation(Name, FKTranslation, Language)
        values ('Artist', 2, 'EN');
        INSERT INTO RoleTranslation(Name, FKTranslation, Language)
        values ('Künstler', 2, 'DE');

        INSERT INTO Role default
        values;
        INSERT INTO RoleTranslation(Name, FKTranslation, Language)
        values ('Actor', 3, 'EN');
        INSERT INTO RoleTranslation(Name, FKTranslation, Language)
        values ('Schauspieler', 3, 'DE');

        INSERT INTO Role default
        values;
        INSERT INTO RoleTranslation(Name, FKTranslation, Language)
        values ('Voice actor', 4, 'EN');
        INSERT INTO RoleTranslation(Name, FKTranslation, Language)
        values ('Synchronsprecher', 4, 'DE');

        INSERT INTO Role default
        values;
        INSERT INTO RoleTranslation(Name, FKTranslation, Language)
        values ('Sound designer', 5, 'EN');
        INSERT INTO RoleTranslation(Name, FKTranslation, Language)
        values ('Sounddesigner', 5, 'DE');

        INSERT INTO Role default
        values;
        INSERT INTO RoleTranslation(Name, FKTranslation, Language)
        values ('Producer', 6, 'EN');
        INSERT INTO RoleTranslation(Name, FKTranslation, Language)
        values ('Produzent', 6, 'DE');

        INSERT INTO Role default
        values;
        INSERT INTO RoleTranslation(Name, FKTranslation, Language)
        values ('Writer', 7, 'EN');
        INSERT INTO RoleTranslation(Name, FKTranslation, Language)
        values ('Schriftsteller', 7, 'DE');

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

            FKTranslation int          not null,
            Language      language     not null,
            foreign key (FKTranslation) references Character (Id),
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

            FKTranslation int      not null,
            Language      language not null,
            foreign key (FKTranslation) references Person (Id),
            primary key (FKTranslation, Language)
        );
        create table PersonXRole
        (
            FKPerson int not null,
            FKRole   int not null,
            foreign key (FKRole) references Role (Id),
            foreign key (FKPerson) references Person (Id),
            primary key (FKPerson, FKRole)
        );

        create table Movie
        (
            Id         int primary key generated always as identity,
            Airing     date,
            Length     interval,
            Score      decimal(5, 2) CHECK (Score BETWEEN 0.99 AND 10.01),
            FKCover    int  not null references Image (Id),
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

            FKTranslation int          not null references Movie (Id),
            Language      language     not null,
            primary key (FKTranslation, Language)
        );

        create table MovieXGenre
        (
            FKMovie int not null references Movie (Id),
            FKGenre int not null references Genre (Id),
            primary key (FKMovie, FKGenre)
        );
        create table MovieXTheme
        (
            FKMovie int not null references Movie (Id),
            FKTheme int not null references Theme (Id),
            primary key (FKMovie, FKTheme)
        );
        create table MovieXCreator
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
            Score        decimal(5, 2) CHECK (Score BETWEEN 0.99 AND 10.01),
            Status       status not null,
            FKCover      int    not null references Image (Id),
            Added        date   not null DEFAULT (CURRENT_DATE),
            Rank         int    not null default 0,
            Popularity   int    not null default 0,
            Favorites    int    not null default 0,
            Members      int    not null default 0
        );
        create table GraphicNovelTranslation
        (
            Title         varchar(150) not null,
            Description   varchar(500),

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

            FKTranslation int          not null,
            Language      language     not null,
            foreign key (FKTranslation) references GraphicNovel (Id),
            primary key (FKTranslation, Language)
        );
        create table GraphicNovelChapter
        (
            Chapter              smallint not null,
            FKGraphicNovel       int      not null,
            FKGraphicNovelVolume int,
            Pages                smallint not null,
            Published            date,
            Score                smallint,
            foreign key (FKGraphicNovel) references GraphicNovel (Id),
            foreign key (FKGraphicNovel, FKGraphicNovelVolume) references GraphicNovelVolume (FKGraphicNovel, Volume),
            primary key (FKGraphicNovel, Chapter)
        );

        create table GraphicNovelChapterTranslation
        (
            Title                 varchar(150) not null,
            Description           varchar(500),

            FKGraphicNovel        int          not null,
            FKGraphicNovelChapter int          not null,
            Language              language     not null,
            foreign key (FKGraphicNovel, FKGraphicNovelChapter) references GraphicNovelChapter (FKGraphicNovel, Chapter),
            primary key (FKGraphicNovel, FKGraphicNovelChapter, Language)
        );
        create table GraphicNovelXPublisher
        (
            FKGraphicNovel int not null references GraphicNovel (Id),
            FKPublisher    int not null references Company (Id),
            primary key (FKGraphicNovel, FKPublisher)
        );
        create table GraphicNovelXCharacter
        (
            FKGraphicNovel int not null,
            FKCharacter    int not null,
            foreign key (FKGraphicNovel) references GraphicNovel (Id),
            foreign key (FKCharacter) references Character (Id),
            primary key (FKGraphicNovel, FKCharacter)
        );
        create table GraphicNovelXGenre
        (
            FKGraphicNovel int not null,
            FKGenre        int not null,
            foreign key (FKGraphicNovel) references GraphicNovel (Id),
            foreign key (FKGenre) references Genre (Id),
            primary key (FKGraphicNovel, FKGenre)
        );
        create table GraphicNovelXTheme
        (
            FKGraphicNovel int not null,
            FKTheme        int not null,
            foreign key (FKGraphicNovel) references GraphicNovel (Id),
            foreign key (FKTheme) references Theme (Id),
            primary key (FKGraphicNovel, FkTheme)
        );
        create table GraphicNovelXCreator
        (
            FKGraphicNovel int not null,
            FKPerson       int not null,
            FKRole         int not null,
            foreign key (FKGraphicNovel) references GraphicNovel (Id),
            foreign key (FKPerson) references Person (Id),
            foreign key (FKRole) references Role (Id),
            primary key (FKGraphicNovel, FKPerson, FKRole)
        );
        create table Book
        (
            Id          int primary key generated always as identity,
            Chapters    smallint,
            Pages       smallint,
            Words       int,
            Published   date,
            Score       decimal(5, 2) not null DEFAULT 0,
            FKCover     int           not null references Image (Id),
            Added       date          not null DEFAULT (CURRENT_DATE),
            Rank        int           not null default 0,
            Popularity  int           not null default 0,
            Favorites   int           not null default 0,
            Members     int           not null default 0,
            FKFranchise int references Franchise (Id)
        );
        create table BookTranslation
        (
            Title         varchar(150) not null,
            Description   varchar(500),

            FKTranslation int          not null,
            Language      language     not null,
            foreign key (FKTranslation) references Book (Id),
            primary key (FKTranslation, Language)
        );
        create table BookXCharacter
        (
            FKBook      int not null,
            FKCharacter int not null,
            foreign key (FKBook) references Book (Id),
            foreign key (FKCharacter) references Character (Id),
            primary key (FKBook, FKCharacter)
        );
        create table BookXGenre
        (
            FKBook  int not null,
            FKGenre int not null,
            foreign key (FKBook) references Book (Id),
            foreign key (FKGenre) references Genre (Id),
            primary key (FKBook, FKGenre)
        );
        create table BookXTheme
        (
            FKBook  int not null,
            FKTheme int not null,
            foreign key (FKBook) references Book (Id),
            foreign key (FKTheme) references Theme (Id),
            primary key (FKBook, FKTheme)
        );
        create table BookXCreator
        (
            FKBook   int not null,
            FKRole   int not null,
            FKPerson int not null,
            foreign key (FKBook) references Book (Id),
            foreign key (FKRole) references Role (Id),
            foreign key (FKPerson) references Person (Id),
            primary key (FKBook, FKRole, FKPerson)
        );
        create table Show
        (
            Id          int primary key generated always as identity,
            AiringStart date,
            AiringEnd   date,
            Score       decimal(5, 2) CHECK (Score BETWEEN 0.99 AND 10.01),
            Seasons     smallint,
            Status      status not null,
            FKCover     int    not null references Image (Id),
            Added       date   not null DEFAULT (CURRENT_DATE),
            Rank        int    not null default 0,
            Popularity  int    not null default 0,
            Favorites   int    not null default 0,
            Members     int    not null default 0,
            FKFranchise int references Franchise (Id)
        );
        create table ShowTranslation
        (
            Title         varchar(150) not null,
            Description   varchar(500),

            FKTranslation int          not null,
            Language      language     not null,
            foreign key (FKTranslation) references Show (Id),
            primary key (FKTranslation, Language)
        );
        create table ShowSeason
        (
            Season      smallint unique not null,
            FKShow      int             not null,
            Episodes    smallint,
            AiringStart date,
            AiringEnd   date,
            Score       smallint,
            foreign key (FKShow) references Show (Id),
            primary key (FKShow, Season)
        );
        create table ShowSeasonTranslation
        (
            Title        varchar(150) not null,
            Description  varchar(500),

            FKShow       int          not null,
            FKShowSeason int          not null,
            Language     language     not null,
            foreign key (FKShow, FKShowSeason) references ShowSeason (FKShow, Season),
            primary key (FKShow, FKShowSeason, Language)
        );
        create table ShowEpisode
        (
            Episode  smallint unique not null,
            FKShow   int             not null references Show (Id),
            FKSeason int, -- Null, since not every episode is part of a season
            Length   smallint,
            Airing   date,
            Score    smallint,
            FKCover  int references Image (Id),
            foreign key (FKShow, FKSeason) references ShowSeason (FKShow, Season),
            primary key (FKShow, Episode)
        );
        create table ShowEpisodeTranslation
        (
            Title         varchar(150) not null,
            Description   varchar(500),

            FKShow        int          not null,
            FKShowEpisode int          not null,
            Language      language     not null,
            foreign key (FKShow, FKShowEpisode) references ShowEpisode (FKShow, Episode),
            primary key (FKShow, FKShowEpisode, Language)
        );
        create table ShowXCharacter
        (
            FKShow      int not null references Show (Id),
            FKCharacter int not null references Character (Id),
            primary key (FKShow, FKCharacter)
        );
        create table ShowXGenre
        (
            FKShow  int not null references Show (Id),
            FKGenre int not null references Genre (Id),
            primary key (FKShow, FKGenre)
        );
        create table ShowXTheme
        (
            FKShow  int not null references Show (Id),
            FKTheme int not null references Theme (Id),
            primary key (FKShow, FKTheme)
        );
        create table ShowXCreator
        (
            FKShow   int not null references Show (Id),
            FKRole   int not null references Role (Id),
            FKPerson int not null references Person (Id),
            primary key (FKShow, FKRole, FKPerson)
        );
        create table Game
        (
            Id          int primary key generated always as identity,
            Published   date,
            Score       decimal(5, 2) CHECK (Score BETWEEN 0.99 AND 10.01),
            FKCover     int  not null references Image (Id),
            Added       date not null DEFAULT (CURRENT_DATE),
            Rank        int  not null default 0, -- Values should not be null,
            Popularity  int  not null default 0, -- because they will only not have a value right after creation
            Favorites   int  not null default 0,
            Members     int  not null default 0,
            FKFranchise int references Franchise (Id)
        );
        create table GameTranslation
        (
            Title         varchar(150) not null,
            Description   varchar(500),
            FKTranslation int          not null,
            Language      language     not null,
            foreign key (FKTranslation) references Game (Id),
            primary key (FKTranslation, Language)
        );
        create table GameXPlatform
        (
            FKGame     int not null,
            FKPlatform int not null,
            foreign key (FKGame) references Game (Id),
            foreign key (FKPlatform) references Platform (Id),
            primary key (FKGame, FKPlatform)
        );
        create table GameXCharacter
        (
            FKGame      int not null references Game (Id),
            FKCharacter int not null references Character (Id),
            primary key (FKGame, FKCharacter)
        );
        create table GameXGenre
        (
            FKGame  int not null references Game (Id),
            FKGenre int not null references Genre (Id),
            primary key (FKGame, FKGenre)
        );
        create table GameXTheme
        (
            FKGame  int not null references Game (Id),
            FKTheme int not null references Theme (Id),
            primary key (FKGame, FKTheme)
        );
        create table GameXCreator
        (
            FKGame   int not null references Game (Id),
            FKRole   int not null references Role (Id),
            FKPerson int not null references Person (Id),
            primary key (FKGame, FKRole, FKPerson)
        );
        create table "User"
        (
            Id               int primary key generated always as identity,
            Name             varchar(50) not null,
            Joined           date        not null DEFAULT (CURRENT_DATE),
            Description      varchar(500),
            FKProfilePicture int references Image (Id),
            Deleted          boolean     not null
        );
        create table UserAverage
        (
            Id                  int primary key generated always as identity,
            FKUser              int,
            GraphicNovelAverage decimal(5, 2) CHECK (GraphicNovelAverage BETWEEN 0.99 AND 10.01),
            ShowAverage         decimal(5, 2) CHECK (ShowAverage BETWEEN 0.99 AND 10.01),
            MovieAverage        decimal(5, 2) CHECK (MovieAverage BETWEEN 0.99 AND 10.01),
            BookAverage         decimal(5, 2) CHECK (BookAverage BETWEEN 0.99 AND 10.01),
            GameAverage         decimal(5, 2) CHECK (GameAverage BETWEEN 0.99 AND 10.01),
            foreign key (FKUser) references "User" (Id)
        );
        create table UserXGraphicNovel
        (
            FKUser         int        not null,
            FKGraphicNovel int        not null,
            UserStatus     userstatus not null,
            Favorite       boolean    not null,
            Score          smallint,
            Review         varchar(255),
            Start          date,
            Finished       date,
            Chapters       smallint,
            Added          date       not null DEFAULT (CURRENT_DATE),
            foreign key (FKUser) references "User" (Id),
            foreign key (FKGraphicNovel) references GraphicNovel (Id),
            primary key (FKUser, FKGraphicNovel)
        );
        create table UserXBook
        (
            FKUser     int        not null,
            FKBook     int        not null,
            UserStatus userstatus not null,
            Favorite   boolean    not null,
            Score      smallint,
            Review     varchar(255),
            Start      date,
            Finished   date,
            Chapters   smallint,
            Pages      smallint,
            Added      date       not null DEFAULT (CURRENT_DATE),
            foreign key (FKUser) references "User" (Id),
            foreign key (FKBook) references Book (Id),
            primary key (FKUser, FKBook)
        );
        create table UserXShow
        (
            FKUser     int        not null,
            FKShow     int        not null,
            UserStatus userstatus not null,
            Favorite   boolean    not null,
            Score      smallint,
            Review     varchar(255),
            Start      date,
            Finished   date,
            Episodes   smallint,
            Added      date       not null DEFAULT (CURRENT_DATE),
            foreign key (FKUser) references "User" (Id),
            foreign key (FKShow) references Show (Id),
            primary key (FKUser, FKShow)
        );
        create table UserXMovie
        (
            FKUser     int        not null,
            FKMovie    int        not null,
            UserStatus userstatus not null,
            Favorite   boolean    not null,
            Score      smallint,
            Review     varchar(255),
            Watched    date,
            Added      date       not null DEFAULT (CURRENT_DATE),
            foreign key (FKUser) references "User" (Id),
            foreign key (FKMovie) references Movie (Id),
            primary key (FKUser, FKMovie)
        );
        create table UserXGame
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
            FKUser       int  not null,
            FKSecondUser int  not null,
            Added        date not null DEFAULT (CURRENT_DATE),
            foreign key (FKUser) references "User" (Id),
            foreign key (FKSecondUser) references "User" (Id),
            primary key (FKUser, FKSecondUser)
        );
        create table Account
        (
            FKUser   int                 not null primary key references "User" (Id),
            EMail    varchar(255) unique not null,
            Password varchar(255)        not null
        );
        create index AccountEmailIndex on Account using HASH (EMail);

    END
$$;