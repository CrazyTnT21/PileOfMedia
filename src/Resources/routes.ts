import {Routes} from '@angular/router';
import {AlbumsComponent} from '../app/pages/albums/albums.component';
import {AnimeComponent} from '../app/pages/anime/anime.component';
import {BooksComponent} from '../app/pages/books/books.component';
import {ComicsComponent} from '../app/pages/comics/comics.component';
import {EditorComponent} from '../app/pages/editor/editor.component';
import {MainComponent} from '../app/pages/main/main.component';
import {MangaComponent} from '../app/pages/manga/manga.component';
import {MoviesComponent} from '../app/pages/movies/movies.component';
import {ProfileComponent} from '../app/pages/profile/profile.component';
import {ShowsComponent} from "../app/pages/shows/shows.component";
import * as StringNames from "../Resources/StringNames.json";
import {ComicPage} from "../app/pages/comics/comicPage";
import {MangaPageComponent} from "../app/pages/manga/MangaPage.component";

const names = StringNames;
export const routes: Routes = [
  {path: "", component: MainComponent},
  {path: names.Manga, component: MangaComponent},
  {path: names.Books, component: BooksComponent},
  {path: names.Comics, component: ComicsComponent},
  {path: names.Comics + "/:id", component: ComicPage},
  {path: names.Manga, component: MangaComponent},
  {path: names.Manga + "/:id", component: MangaPageComponent},
  // {path: names.Comics + ":id/Volumes/:id", component: ComicsComponent},
  //  {path: names.Comics + ":id/Chapter/:id", component: ComicsComponent},
  {path: names.Shows, component: ShowsComponent},
  {path: names.Movies, component: MoviesComponent},
  {path: names.Albums, component: AlbumsComponent},
  {path: names.Anime, component: AnimeComponent},
  {path: names.Profile, component: ProfileComponent},
  {path: names.Editor, component: EditorComponent},
  {path: '**', component: MainComponent}
];
