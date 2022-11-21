import { Component, OnInit } from '@angular/core';

@Component({
  selector: 'app-main',
  templateUrl: './main.component.html'
})
export class MainComponent implements OnInit {

  data: any;

  constructor() {

    this.data = {
      labels: ['January', 'February', 'March', 'April', 'May', 'June', 'July', "October", "September", "November", "December"],
      datasets: [
        {
          label: 'Second Dataset',
          data: [10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
          backgroundColor: [
            "#fcb603",
            "#787366",
            "#3f6cbf",
            "#7f3fbf",
            "#3fa1bf",
            "#1cb603",
            "#187366",
            "#1f6cbf",
            "#1f3fbf",
            "#1fa1bf",
          ],
        }
      ]
    }
  }

  ngOnInit(): void {
  }
  update(event: Event) {
    this.data = event;//create new data
  }

}
