import { HttpClient, HttpHeaders, HttpParams } from '@angular/common/http';
import { Component, OnInit } from '@angular/core';
import { Observable } from 'rxjs';
import { Base } from 'src/app/mainapp/Base';
@Component({
  selector: 'app-books',
  templateUrl: './books.component.html'
})
export class BooksComponent extends Base implements OnInit {

  items: Observable<any[]>;
  constructor(private httpClient: HttpClient) {
    super();
    let httpHeaders = new HttpHeaders()
      .set('Content-Type', 'application/json')
      .set('Authorization', 'Basic QWxhZGRpb');
    let httpParams = new HttpParams()
      .set('title', "First");
    let header = new HttpHeaders();
    this.items = this.httpClient
      .get<any[]>('http://localhost:8000/api/items', {
        headers: httpHeaders,
        params: httpParams
      });

  }
  test() {
    console.log(this.items);
  }
  ngOnInit() {
  }
}

interface testitem {
  PKItem: number;
  FKCategory: Number;
  IName: string;
  IDescription: string;
  FKSeries: string;
  IImageSource: string;
  IStartDate: string;
  IEndDate: string;
}
