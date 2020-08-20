import { Component, ViewChild, ElementRef } from '@angular/core';
import { DomSanitizer, SafeUrl, SafeResourceUrl } from '@angular/platform-browser';

import { HttpClient, HttpParams } from '@angular/common/http';

import { Observable, from } from 'rxjs';
import { catchError } from 'rxjs/operators';

import { RespData } from './app';
import { stringify } from '@angular/compiler/src/util';
@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css']
})



export class AppComponent {
  @ViewChild('img') imgChild: ElementRef;

  msg = '';
  data = "data:image/gif;base64,R0lGODlhEAAQAMQAAORHHOVSKudfOulrSOp3WOyDZu6QdvCchPGolfO0o/XBs/fNwfjZ0frl3/zy7////wAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAACH5BAkAABAALAAAAAAQABAAAAVVICSOZGlCQAosJ6mu7fiyZeKqNKToQGDsM8hBADgUXoGAiqhSvp5QAnQKGIgUhwFUYLCVDFCrKUE1lBavAViFIDlTImbKC5Gm2hB0SlBCBMQiB0UjIQA7";
  imgData: SafeResourceUrl;
  isReload = false;

  constructor(
    private sanitizer: DomSanitizer,
    private http: HttpClient
  ) {
    this.imgData = sanitizer.bypassSecurityTrustResourceUrl(this.data)
  }

  setQrCode(img: string) {
    this.imgData = this.sanitizer.bypassSecurityTrustResourceUrl(this.data)
  }

  ngOnInit() {
    this.getImageData()
  }

  ngAfterViewInit(): void {
    // DOM节点
    console.log(this.imgChild.nativeElement);
  }

  url = '/passports';


  getImageData(): string {
    this.http.get('http://127.0.0.1:1233/passports', {
      params: {
        "reload": stringify(this.isReload)
      }
    })
      .subscribe({
        next(response) { console.log(response); },
        error(err) { console.error('Error: ' + err); },
        complete() { console.log('Completed'); }
      });
      return ''
  }


}
