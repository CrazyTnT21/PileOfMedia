import {Pipe, PipeTransform} from '@angular/core';

/*
 * Formats the value using the given function and formatting
 * e.g:
 *
 * formatting = Chapters: {}
 * rowvalue = 12
 * result = Chapters: 12
 * Usage:
 *   value | formatting:formatting
 * Example:
 *   {{ 2 | exponentialStrength:10 }}
 *   formats to: 1024
*/
@Pipe({name: 'Wrapper'})
export class WrapperPipe implements PipeTransform {

    transform(value: any, args?: PipeTransform[]): any {
        if (!args)
            return;
        console.log(args)
        let all = [];
        for (let i = 0; i < args.length; i++) {
            if (args[i] === undefined)
                return;
            all.push(args[i].transform(value));
        }
        console.log(all.join(" "));
        return all.join(" ");
    }
}