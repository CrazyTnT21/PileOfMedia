import { Component } from '@angular/core';

@Component({
  selector: 'app-editor',
  templateUrl: './editor.component.html'
})
export class EditorComponent {
  custom: boolean;
  comtype: typeof Componenttype = Componenttype;
  components: string[] = [
    "Component",
    "GameObject",
    "UIObject"
  ];
  selectedindex: number = 0;
  GameItems: GameComponent[] = [
    {
      Name: "Test",
      Type: "Component",
      parameters: { Position: [2, 1.25], Color: "", Active: true, Size: [1, 1], Rotation: 0 }
    },
    {
      Name: "Test2",
      Type: "GameObject",
      parameters: { Position: [2, 1.25], Color: "", Active: true, Size: [1, 1], Rotation: 0 }
    }
  ];
  constructor() { }
  testclick(value: number) {
    this.selectedindex = value;
    console.log(this.GameItems[this.selectedindex]);
  }
  createGameItem() {
    let count: number = 0;
    const item: GameComponent = new GameComponent();
    for (let i = 0; i < this.GameItems.length; i++)
      if (this.GameItems[i].Name == item.Name + `(${count})` || this.GameItems[i].Name == item.Name) {
        count++;
      }
    if (count > 0)
      item.Name += `(${count})`;

    this.selectedindex = this.GameItems.length;
    this.GameItems.push(item);
  }
  removeGameItem() {
    if (this.GameItems.length > 0) {
      this.GameItems.splice(this.selectedindex, 1);
      this.selectedindex = this.GameItems.length - 1;
    }
  }
  log(value: any) {
    console.log(value);
  }
  generateService() {

    const data: any = this.generateCS(this.GameItems);
    const blob = new Blob([data], { type: 'text' });
    const url = URL.createObjectURL(blob);
    const link = document.createElement("a"); // Or maybe get it from the current document
    link.href = url;
    link.download = "Project.cs";
    window.open(url);
    //  link.click();

  }
  generateCS(value: GameComponent[]): string {
    let result: string = "";
    const using: string = "Microsoft.Xna.Framework";
    const namespace: string = "Monogame-Game";
    result += "using " + using + ";";
    result += "\n\nnamespace " + namespace + "\n{\n";
    result += "\tpublic partial class Menu\n\t{";
    for (let i = 0; i < value.length; i++) {
      result += "\n\t\t";
      result += "private " + value[i].Type;
      result += " ";
      result += value[i].Name;
      result += " = new ";
      result += value[i].Type.toString();
      result += "() {";
      result += " Position = " + this.toVector2(value[i].parameters.Position as any);
      // result += ", "
      result += " };";
    }
    result += "\n\t}";
    result += "\n}";
    return result;
  }
  toVector2(value: number[]): string {
    return "new Vector2(" + value[0] + ", " + value[1] + ")";
  }
  projectdataLoad(event: Event) {
    console.log(event);
  }
}
export class GameComponent {
  Name: string = "GameItem";
  Type: string = "Component";
  parameters: Parameters = new Parameters();
}
class Parameters {
  Position: number[] = [0, 0];
  Color: string;
  Size: number[] = [1, 1];
  Children?: GameComponent[];
  Rotation: Number = 0;
  Active: boolean = true;
}
enum Componenttype {
  GameObject,
  Component,
  UIObject
}
