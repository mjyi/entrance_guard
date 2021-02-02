import { Component } from '@angular/core';

import { AppService } from './app.service';


@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css']
})



export class AppComponent {

  data: string;

  constructor(private appService: AppService) { 
    this.data = "";
  }

  ngOnInit() {
    this.getImageData(false)
  }

  reloadQrCode() {
    this.getImageData(true)
  }

  getImageData(isReload: boolean) {
    this.appService.getImageData(isReload)
      .subscribe(data => (this.data = data.data))
  }
}
