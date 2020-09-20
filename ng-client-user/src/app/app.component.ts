import { Component, OnInit, OnDestroy } from '@angular/core';
import { Meta } from '@angular/platform-browser';
import { Subject } from 'rxjs';
import { takeUntil } from 'rxjs/operators';
import { AuthService } from 'src/auth/services/auth.service';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss'],
})
export class AppComponent implements OnInit, OnDestroy {
  ngUnsubscribe = new Subject<void>();
  title = 'ng-user-client';
  loggedIn = false;
  displayName: string;

  constructor(private authService: AuthService, private meta: Meta) {}

  ngOnInit() {
    /*
    this.meta.updateTag({ name: `title`, content: `Axmouth's Website` });
    this.meta.updateTag({
      name: `description`,
      content: `Axmouth's Website, featuring some projects and posts related to them`,
    });
    this.meta.updateTag({ name: `keywords`, content: `axmouth,developer,webdev,programmer,portfolio` });
    this.meta.updateTag({ name: `author`, content: `Axmouth` });
    // Open Graph / Facebook
    this.meta.updateTag({ property: `og:url`, content: `https://axmouth.dev/` });
    this.meta.updateTag({ property: `og:title`, content: `Axmouth's Website` });
    this.meta.updateTag({
      property: `og:description`,
      content: `Axmouth's Website, featuring some projects and posts related to them`,
    });
    this.meta.updateTag({ property: `og:image`, content: `https://axmouth.dev/assets/gggg.png` });
    // Twitter
    this.meta.updateTag({ property: `twitter:card`, content: `https://axmouth.dev/assets/gggg.png` });
    this.meta.updateTag({ property: `twitter:url`, content: `https://axmouth.dev/` });
    this.meta.updateTag({ property: `twitter:title`, content: `Axmouth's Website` });
    this.meta.updateTag({
      property: `twitter:description`,
      content: `Axmouth's Website, featuring some projects and posts related to them`,
    });
    this.meta.updateTag({ property: `twitter:image`, content: `https://axmouth.dev/assets/gggg.png` });
    /*
    this.meta.addTags([
      { name: `title`, content: `Axmouth's Website` },
      { name: `description`, content: `Axmouth's Website, featuring some projects and posts related to them` },
      { name: `keywords`, content: `axmouth,developer,webdev,programmer,portfolio` },
      { name: `author`, content: `Axmouth` },
      // Open Graph / Facebook
      { property: `og:url`, content: `https://axmouth.dev/` },
      { property: `og:title`, content: `Axmouth's Website` },
      { property: `og:description`, content: `Axmouth's Website, featuring some projects and posts related to them` },
      { property: `og:image`, content: `https://axmouth.dev/assets/gggg.png` },
      // Twitter
      { property: `twitter:card`, content: `https://axmouth.dev/assets/gggg.png` },
      { property: `twitter:url`, content: `https://axmouth.dev/` },
      { property: `twitter:title`, content: `Axmouth's Website` },
      {
        property: `twitter:description`,
        content: `Axmouth's Website, featuring some projects and posts related to them`,
      },
      { property: `twitter:image`, content: `https://axmouth.dev/assets/gggg.png` },
    ]);*/
    this.authService
      .isAuthenticatedOrRefresh()
      .pipe(takeUntil(this.ngUnsubscribe))
      .subscribe((result) => {
        this.loggedIn = result;
        if (result === true) {
          this.authService.getUsername().subscribe((name) => {
            this.displayName = name;
          });
          this.authService.getToken().subscribe((token) => console.log);
        }
      });
    this.authService
      .onAuthenticationChange()
      .pipe(takeUntil(this.ngUnsubscribe))
      .subscribe((result) => {
        this.loggedIn = result;
        if (result === true) {
          this.authService.getUsername().subscribe((name) => {
            this.displayName = name;
          });
        }
      });
  }

  ngOnDestroy(): void {
    this.ngUnsubscribe.next();
    this.ngUnsubscribe.complete();
  }
}
