import { NgModule } from '@angular/core';
import { RouterModule } from '@angular/router';
import { routes } from '../../Resources/other/routes';
import { TaskbarComponent } from '../../Resources/templates/taskbar.component';

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule],

  providers: [TaskbarComponent]
})
export class AppRoutingModule { }
