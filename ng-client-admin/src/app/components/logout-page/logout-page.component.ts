import { Component, OnInit, OnDestroy } from '@angular/core';
import { AuthService } from '../../../auth/services/auth.service';
import { Router } from '@angular/router';
import { Title } from '@angular/platform-browser';
import { takeUntil } from 'rxjs/operators';
import { Subject } from 'rxjs';

@Component({
  selector: 'app-logout-page',
  templateUrl: './logout-page.component.html',
  styleUrls: ['./logout-page.component.scss'],
})
export class LogoutPageComponent implements OnInit, OnDestroy {
  ngUnsubscribe = new Subject<void>();

  constructor(private authService: AuthService, private router: Router, private title: Title) {}

  async ngOnInit(): Promise<void> {
    this.title.setTitle(`Logging Out - Axmouth's Website Admin Site`);
    this.authService
      .logout()
      .pipe(takeUntil(this.ngUnsubscribe))
      .subscribe((res) => {
        console.log(res);
        this.router.navigate(['/']);
      });
  }

  ngOnDestroy(): void {
    this.ngUnsubscribe.next();
    this.ngUnsubscribe.complete();
  }
}
