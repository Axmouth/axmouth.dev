import { Injectable } from '@angular/core';
import { Router, ActivatedRoute, NavigationEnd } from '@angular/router';
import { filter } from 'rxjs/operators';

@Injectable({
  providedIn: 'root',
})
export class RouteStateService {
  previousUrl = '';
  currentUrl = '';
  history: string[] = [''];

  constructor(router: Router, private route: ActivatedRoute) {
    router.events.pipe(filter((event) => event instanceof NavigationEnd)).subscribe((e: any) => {
      if (e?.url?.includes && e.url.includes('logout')) {
        return;
      }
      this.previousUrl = this.currentUrl;
      this.currentUrl = e.url;
      this.history.push(e.url);
    });
  }

  getPreviousUrl() {
    return this.route.snapshot.queryParams.returnUrl || this.previousUrl || '';
    // return history[history.length - 2] || '';
  }

  getHistory() {
    return this.history;
  }
}
