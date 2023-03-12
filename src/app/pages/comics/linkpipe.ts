import {Pipe, PipeTransform} from "@angular/core";

@Pipe({
  name: "text"
})
export class Linkpipe implements PipeTransform{

  transform(value: string): string {
    const result = value.match(new RegExp("test([\\s\\S]*?)1"));
    if (!result)
      return "";
    let test = "";
    for (let i = 0; i < result.length; i++){
      test += "<a href='test'>test</a>";
    }
    return test;
  }

}
