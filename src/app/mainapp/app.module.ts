import { NgModule } from '@angular/core';
import { AppComponent } from './app.component';
import { declarations } from 'src/Resources/other/declarations';
import { imports } from 'src/Resources/other/imports';
import { providers } from 'src/Resources/other/providers';

@NgModule({
  declarations: declarations,
  imports: imports,
  providers: providers,
  bootstrap: [AppComponent]
})
export class AppModule { }
