import {AppComponent} from "src/app/mainapp/app.component";
import {BooksComponent} from "src/app/pages/books/books.component";
import {ComicsComponent} from "src/app/pages/comics/comics.component";
import {MainComponent} from "src/app/pages/main/main.component";
import {MangaComponent} from "src/app/pages/manga/manga.component";
import {MoviesComponent} from "src/app/pages/movies/movies.component";
import {ShowsComponent} from "src/app/pages/shows/shows.component";
import {TaskbarComponent} from "src/Resources/templates/taskbar.component";
import {AlbumsComponent} from "src/app/pages/albums/albums.component";
import {AnimeComponent} from "src/app/pages/anime/anime.component";
import {ProfileComponent} from "src/app/pages/profile/profile.component";
import {table} from "src/Resources/templates/table.component";
import {CellEdit} from "src/Resources/templates/CellEdit";
import {EditorComponent} from "src/app/pages/editor/editor.component";
import {ComicPage} from "src/app/pages/comics/comicPage";
import {RowComponent} from "src/Resources/templates/row.component";
import {CardComponent} from "src/Resources/templates/card.component";

export const declarations: any = [
    AppComponent,
    MainComponent,
    TaskbarComponent,
    MangaComponent,
    ComicsComponent,
    ShowsComponent,
    MoviesComponent,
    BooksComponent,
    AlbumsComponent,
    AnimeComponent,
    ProfileComponent,
    table,
    EditorComponent,
    CellEdit,
    ComicPage,
    RowComponent,
    CardComponent
];