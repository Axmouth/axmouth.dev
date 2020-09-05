import { Component, OnInit } from '@angular/core';
import { AuthService } from '../../../auth/services/auth.service';

@Component({
  selector: 'app-navbar',
  templateUrl: './navbar.component.html',
  styleUrls: ['./navbar.component.scss'],
})
export class NavbarComponent implements OnInit {
  loggedIn = false;

  constructor(private authService: AuthService) {}

  ngOnInit(): void {
    this.authService.onAuthenticationChange().subscribe((result) => {
      this.loggedIn = result;
    });
  }
}
