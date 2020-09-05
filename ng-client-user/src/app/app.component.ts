import { Component, OnInit } from '@angular/core';
import { AuthService } from 'src/auth/services/auth.service';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss'],
})
export class AppComponent implements OnInit {
  title = 'ng-user-client';
  loggedIn = false;
  displayName: string;

  constructor(private authService: AuthService) {}

  ngOnInit() {
    this.authService.isAuthenticatedOrRefresh().subscribe((result) => {
      this.loggedIn = result;
      if (result === true) {
        this.authService.getUsername().subscribe((name) => {
          this.displayName = name;
        });
        this.authService.getToken().subscribe((token) => console.log);
      }
    });
    this.authService.onAuthenticationChange().subscribe((result) => {
      this.loggedIn = result;
      if (result === true) {
        this.authService.getUsername().subscribe((name) => {
          this.displayName = name;
        });
      }
    });
  }
}
