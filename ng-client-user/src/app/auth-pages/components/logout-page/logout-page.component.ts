import { Component, OnInit } from '@angular/core';
import { AuthService } from 'src/auth';
import { Router } from '@angular/router';
import { RouteStateService } from 'src/app/shared/services/route-state.service';
import { Title } from '@angular/platform-browser';

@Component({
  selector: 'app-logout-page',
  templateUrl: './logout-page.component.html',
  styleUrls: ['./logout-page.component.scss'],
})
export class LogoutPageComponent implements OnInit {
  constructor(
    private authService: AuthService,
    private router: Router,
    private routeStateService: RouteStateService,
    private title: Title,
  ) {}

  async ngOnInit(): Promise<void> {
    this.title.setTitle('axmouth.dev - Logging Out');
    this.authService.logout().subscribe(async (res) => {
      console.log(res);
      await this.router.navigateByUrl(this.routeStateService.getPreviousUrl());
    });
  }
}
