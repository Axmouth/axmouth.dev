import { Component, OnInit } from '@angular/core';
import { AuthService } from '../../../auth/services/auth.service';
import { Router } from '@angular/router';

@Component({
  selector: 'app-logout-page',
  templateUrl: './logout-page.component.html',
  styleUrls: ['./logout-page.component.scss'],
})
export class LogoutPageComponent implements OnInit {
  constructor(private authService: AuthService, private router: Router) {}

  async ngOnInit(): Promise<void> {
    this.authService.logout().subscribe((res) => {
      console.log(res);
      this.router.navigate(['/']);
    });
  }
}
