import { NgModule } from '@angular/core';
import {declarations} from "../Resources/declarations";
import {imports} from "../Resources/imports";
import {providers} from "../Resources/providers";
import {AppComponent} from "./app.component";

@NgModule({
  declarations: declarations,
  imports: imports,
  providers: providers,
  bootstrap: [AppComponent]
})
export class AppModule { }
