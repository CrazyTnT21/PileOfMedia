import {Component, OnInit, TemplateRef} from '@angular/core';

@Component({
  selector: 'app-main',
  templateUrl: './main.component.html'
})
export class MainComponent implements OnInit {

  data: any;
  constructor() {
  }
  closeDialog(dialog: HTMLDialogElement){
      dialog.close();
  }
   openDialog(dialog: HTMLDialogElement){
    //  dialog.open = !dialog.open;
       dialog.showModal();
 //   this.dialog.open(templateref,{width:"200px"});
   }

  ngOnInit(): void {
  }
  update(event: Event) {
    this.data = event;//create new data
  }

}

