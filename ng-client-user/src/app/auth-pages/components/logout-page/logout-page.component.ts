import { Component, OnInit, OnDestroy } from '@angular/core';
import { AuthService } from 'src/auth';
import { Router } from '@angular/router';
import { RouteStateService } from 'src/app/shared/services/route-state.service';
import { Meta, Title } from '@angular/platform-browser';
import { Subject } from 'rxjs';
import { takeUntil } from 'rxjs/operators';

@Component({
  selector: 'app-logout-page',
  templateUrl: './logout-page.component.html',
  styleUrls: ['./logout-page.component.scss'],
})
export class LogoutPageComponent implements OnInit, OnDestroy {
  ngUnsubscribe = new Subject<void>();

  constructor(
    private authService: AuthService,
    private router: Router,
    private routeStateService: RouteStateService,
    private title: Title,
    private meta: Meta,
  ) {}

  async ngOnInit(): Promise<void> {
    this.title.setTitle(`Logging Out - Axmouth's Websitte`);
    this.meta.updateTag({ name: `title`, content: this.title.getTitle() });
    this.meta.updateTag({ property: `og:url`, content: window.location.href });
    this.meta.updateTag({ property: `og:title`, content: this.title.getTitle() });
    this.meta.updateTag({ property: `twitter:url`, content: window.location.href });
    this.meta.updateTag({ property: `twitter:title`, content: this.title.getTitle() });
    this.authService
      .logout()
      .pipe(takeUntil(this.ngUnsubscribe))
      .subscribe(async (res) => {
        console.log(res);
        await this.router.navigateByUrl(this.routeStateService.getPreviousUrl());
      });
  }

  ngOnDestroy(): void {
    this.ngUnsubscribe.next();
    this.ngUnsubscribe.complete();
  }
}
