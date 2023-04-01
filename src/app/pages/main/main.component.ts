import {Component, OnInit, ViewChild} from '@angular/core';
import {DialogComponent} from "Resources/Templates/dialog.component";
import {HTTPRequester} from "Resources/HttpRequester";
import {HttpParams} from "@angular/common/http";
import {Base} from "../../../Resources/Base";
import {Config} from "../../../Resources/Config";

@Component({
  selector: 'app-main',
  templateUrl: './main.component.html'
})
export class MainComponent extends Base implements OnInit {
  public static config : Config = MainComponent.LoadConfig();
  user: any = {};
  @ViewChild(DialogComponent)
  dialog: DialogComponent;
  data: any;

  constructor() {
    super();

    this.user = JSON.parse(localStorage.getItem('user') as string);
    if (!this.user)
      this.user = {};

  }
  public static LoadConfig(): Config {
    const data = localStorage.getItem("Preference");
    console.log(data);
    if (data)
      return JSON.parse(data);
    return new Config();
  }
  public static saveConfig() {
    localStorage.setItem("Preference", JSON.stringify(MainComponent.config));
  }

  openDialog(dialog: HTMLDialogElement) {
    //  dialog.open = !dialog.open;
    dialog.showModal();
    //   this.dialog.open(templateref,{width:"200px"});
  }

  async createUser() {
    localStorage.setItem('user', JSON.stringify(this.user));
    await HTTPRequester.Post("api/user", new HttpParams(), {rows: [this.user]});
    console.log("createuser");
  }

  ngOnInit(): void {
  }

  update(event: Event) {
    this.data = event;//create new data
  }
}

