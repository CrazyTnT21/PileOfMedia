import {AppComponent} from "../app/app.component";
import {BooksComponent} from "../app/pages/books/books.component";
import {ComicsComponent} from "../app/pages/comics/comics.component";
import {MainComponent} from "../app/pages/main/main.component";
import {MangaComponent} from "../app/pages/manga/manga.component";
import {MoviesComponent} from "../app/pages/movies/movies.component";
import {ShowsComponent} from "../app/pages/shows/shows.component";
import {TaskbarComponent} from "../Resources/Templates/taskbar.component";
import {AlbumsComponent} from "../app/pages/albums/albums.component";
import {AnimeComponent} from "../app/pages/anime/anime.component";
import {ProfileComponent} from "../app/pages/profile/profile.component";
import {table} from "../Resources/Templates/table.component";
import {CellEdit} from "../Resources/Templates/CellEdit";
import {EditorComponent} from "../app/pages/editor/editor.component";
import {ComicPage} from "../app/pages/comics/comicPage";
import {RowComponent} from "../Resources/row.component";
import {CardComponent} from "../Resources/Templates/card.component";
import {LoginComponent} from "../Resources/Templates/Login";
import {DialogComponent} from "../Resources/Templates/dialog.component";
import {FooterComponent} from "../Resources/Templates/footer.component";
import {AdminComponent} from "../app/pages/admin/admin.component";
import {FormattingPipe} from "./pipes/formatting";
import {WrapperPipe} from "./pipes/wrapperPipe";

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
    CardComponent,
    LoginComponent,
    DialogComponent,
    FooterComponent,
    AdminComponent,
    FormattingPipe,
    WrapperPipe
];