import {Component, Input} from '@angular/core';
import {Base} from 'src/app/mainapp/Base';

@Component({
    selector: 'app-taskbar',
    template: `
        <!--            <img src="data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCAyNTAgMjUwIj4KICAgIDxwYXRoIGZpbGw9IiNERDAwMzEiIGQ9Ik0xMjUgMzBMMzEuOSA2My4ybDE0LjIgMTIzLjFMMTI1IDIzMGw3OC45LTQzLjcgMTQuMi0xMjMuMXoiIC8+CiAgICA8cGF0aCBmaWxsPSIjQzMwMDJGIiBkPSJNMTI1IDMwdjIyLjItLjFWMjMwbDc4LjktNDMuNyAxNC4yLTEyMy4xTDEyNSAzMHoiIC8+CiAgICA8cGF0aCAgZmlsbD0iI0ZGRkZGRiIgZD0iTTEyNSA1Mi4xTDY2LjggMTgyLjZoMjEuN2wxMS43LTI5LjJoNDkuNGwxMS43IDI5LjJIMTgzTDEyNSA1Mi4xem0xNyA4My4zaC0zNGwxNy00MC45IDE3IDQwLjl6IiAvPgogIDwvc3ZnPg=="-->
        <!--                 class="max"/>-->
        <div class="col-12 mb">
            <div class="dropdown col" *ngFor="let page of Pages">
                <button class="fill"
                        (click)="tools.navigatePage(page.url)">
                    <div *ngIf="page.name == null">{{page.url}}</div>
                    <div *ngIf="page.name != null">{{page.name}}</div>
                </button>
                <div class="dropdown-content col-12">
                    <button *ngFor="let child of page.children" class="fill">
                        <div *ngIf="child.name == null">{{page.url}}</div>
                        <div *ngIf="child.name != null">{{page.name}}</div>
                    </button>
                </div>
            </div>
            <div class="dropdown">
                <button>Settings</button>
                <div class="dropdown-content">
                    <button>Profile</button>
                    <button>About</button>
                    <button (click)="setlanguage()">Language</button>
                </div>
            </div>
        </div>
    `
})
export class TaskbarComponent extends Base {

    @Input("Pages")
    Pages: Page[];

    constructor() {
        super();
    }

    setlanguage() {
        if (Base.Language == "German")
            Base.Language = "English";
        else
            Base.Language = "German";
    }
}

export interface Page {
    icon?: string;
    url: string;
    name?: string;
    children?: Page[];
    query?: any;

}


