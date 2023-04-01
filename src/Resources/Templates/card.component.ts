import {Component, Input} from '@angular/core';

@Component({
  selector: 'app-card',
  template: `

      <div class="overlay">
          <div class="content-parent lazy fix-4">
              <div><img [src]="image" class="fix-4"></div>
<!--              <div class="content pad"><h2>{{title}}</h2></div>-->

<!--              <div class="content fix-4">-->
                  <ng-content></ng-content>
<!--              </div>-->
          </div>
      </div>
      <!--      <ng-container class="fix-4 content-parent">-->
      <!--          <img [src]="image" class="fix-4">-->
      <!--          <ng-container>-->
      <!--              <ng-container class="content">-->
      <!--               test1 <div>test</div>-->

      <!--              </ng-container>-->
      <!--          </ng-container>-->
      <!--      </ng-container>-->
  `,
})
export class CardComponent {
  public _size: number = 5;
//sizes starting from 1-12 using the class fix-1-12
  @Input() size: number = 3;
  @Input() image: string = "Assets/testimg.svg";
@Input() title: string = "Title";
  constructor() {
  }
}


