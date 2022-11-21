import { Routes } from '@angular/router';
import { AlbumsComponent } from 'src/app/pages/albums/albums.component';
import { AnimeComponent } from 'src/app/pages/anime/anime.component';
import { BooksComponent } from 'src/app/pages/books/books.component';
import { ComicPage } from 'src/app/pages/comics/comicPage';
import { ComicsComponent } from 'src/app/pages/comics/comics.component';
import { EditorComponent } from 'src/app/pages/editor/editor.component';
import { MainComponent } from 'src/app/pages/main/main.component';
import { MangaComponent } from 'src/app/pages/manga/manga.component';
import { MoviesComponent } from 'src/app/pages/movies/movies.component';
import { ProfileComponent } from 'src/app/pages/profile/profile.component';
import { ShowsComponent } from 'src/app/pages/shows/shows.component';
import * as StringNames from "src/Resources/other/StringNames.json";

const names = StringNames;
export const routes: Routes = [
    { path: "", component: MainComponent },
    { path: names.Manga, component: MangaComponent },
    { path: names.Books, component: BooksComponent },
    { path: names.Comics, component: ComicsComponent },
    { path: "comicPage", component: ComicPage },
    { path: names.Shows, component: ShowsComponent },
    { path: names.Movies, component: MoviesComponent },
    { path: names.Albums, component: AlbumsComponent },
    { path: names.Anime, component: AnimeComponent },
    { path: names.Profile, component: ProfileComponent },
    { path: names.Editor, component: EditorComponent },
    { path: '**', component: MainComponent }
];