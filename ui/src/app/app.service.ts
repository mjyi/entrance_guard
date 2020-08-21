import { Injectable } from '@angular/core';
import { HttpClient, HttpParams } from '@angular/common/http';
import { HttpHeaders } from '@angular/common/http';

import { Observable, from } from 'rxjs';
import { catchError } from 'rxjs/operators';

import { HttpErrorHandler, HandleError } from './http-error-handler.service';

import { RespData } from './app.data';
import { stringify } from '@angular/compiler/src/util';

import {environment } from '../environments/environment';

@Injectable({
  providedIn: 'root'
})
export class AppService {
  passportsUrl = environment.baseUrl + '/passports';

  private handleError: HandleError;
  

  constructor(
    private http: HttpClient,
    httpErrorHandler: HttpErrorHandler,
  ) {

    this.handleError = httpErrorHandler.createHandleError('AppService');
   }

  getImageData(isReload: boolean): Observable<RespData> {
    return this.http.get<RespData>(this.passportsUrl, {
      params: {
        "reload": stringify(isReload)
      }
    })
    .pipe(
      catchError(this.handleError('', { } as RespData))
    )
  }


}
