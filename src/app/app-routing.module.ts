import { NgModule } from '@angular/core';
import { RouterModule } from '@angular/router';
import { routes } from '../Resources/routes';
import { TaskbarComponent } from '../Resources/Templates/taskbar.component';

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule],

  providers: [TaskbarComponent]
})
export class AppRoutingModule { }
