import { Injectable, Inject, PLATFORM_ID } from '@angular/core';
import { isPlatformBrowser } from '@angular/common';

@Injectable({
  providedIn: 'root',
})
export class IsBrowserService {
  inBrowser: boolean;

  constructor(@Inject(PLATFORM_ID) private platform: Object) {
    this.inBrowser = isPlatformBrowser(platform);
  }

  isInBrowser() {
    return this.inBrowser;
  }
}
