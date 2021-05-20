import { HttpClient } from '@angular/common/http';
import { Inject, Injectable, PLATFORM_ID, OnDestroy } from '@angular/core';
import { Router, Event, NavigationStart, NavigationError, NavigationEnd } from '@angular/router';
import { isPlatformBrowser } from '@angular/common';
import { apiRoot } from 'src/environments/environment';
import { Subject } from 'rxjs';
import { take, takeUntil } from 'rxjs/operators';
import { RestApiService } from './rest-api.service';
import { Response } from 'src/app/models/api/response';

interface LocationResponse {
  country_code: string | null;
  country_name: string | null;
  city: string | null;
  postal: string | null;
  latitude: string | null;
  longitude: string | null;
  IPv4: string | null;
  state: string | null;
}

interface CreatePageViewRequest {
  pageUrl: string;
  latitude: string;
  longitude: string;
  countryCode: string;
}

@Injectable({
  providedIn: 'root',
})
export class PageViewService implements OnDestroy {
  url = `${apiRoot}/page-views`;
  ngUnsubscribe = new Subject<void>();
  location: LocationResponse | null = null;
  locFetch = new Subject<void>();

  constructor(
    private apiService: RestApiService,
    private http: HttpClient,
    router: Router,
    @Inject(PLATFORM_ID) private platform: object,
  ) {
    if (isPlatformBrowser(this.platform)) {
      http.get<LocationResponse>('https://geolocation-db.com/json/').subscribe((result) => {
        this.location = result;
        console.log(result);
        this.locFetch.next();
      });

      router.events.pipe(takeUntil(this.ngUnsubscribe)).subscribe((event: Event) => {
        if (event instanceof NavigationStart) {
          // do something on start activity
          console.log('nav start');
        } else if (event instanceof NavigationError) {
          // Handle error
          console.error(event.error);
        } else if (event instanceof NavigationEnd) {
          // do something on end activity
          console.log('nav end');
          console.log(event.url);
          console.log(event.urlAfterRedirects);
          if (this.location === null) {
            this.locFetch.pipe(take(1)).subscribe(() => {
              this.registerPageView({
                pageUrl: event.url,
                latitude: this.location.latitude,
                longitude: this.location.longitude,
                countryCode: this.location.country_code,
              });
            });
          } else {
            this.registerPageView({
              pageUrl: event.url,
              latitude: this.location.latitude,
              longitude: this.location.longitude,
              countryCode: this.location.country_code,
            });
          }
        }
      });
    }
  }

  registerPageView(req: CreatePageViewRequest): void {
    this.apiService.create<Response<any>>(this.url, req, {}).pipe(takeUntil(this.ngUnsubscribe)).subscribe();
  }

  ngOnDestroy(): void {
    this.ngUnsubscribe.next();
    this.ngUnsubscribe.complete();
  }
}
