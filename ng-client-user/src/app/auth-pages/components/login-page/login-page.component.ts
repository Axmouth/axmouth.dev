import { Component, OnInit } from '@angular/core';
import { FormGroup, FormControl } from '@angular/forms';
import { AuthService } from 'src/auth/services/auth.service';
import { RouteStateService } from 'src/app/shared/services/route-state.service';
import { Router } from '@angular/router';

@Component({
  selector: 'app-login-page',
  templateUrl: './login-page.component.html',
  styleUrls: ['./login-page.component.scss'],
})
export class LoginPageComponent implements OnInit {
  loginForm = new FormGroup({
    email: new FormControl(''),
    password: new FormControl(''),
  });
  errors = [];

  loginInProgress = false;

  constructor(private authService: AuthService, private router: Router, private routeStateService: RouteStateService) {}

  ngOnInit(): void {}

  onSubmit() {
    this.loginInProgress = true;
    this.authService
      .authenticate({
        email: this.loginForm.get('email').value,
        password: this.loginForm.get('password').value,
      })
      .subscribe(
        async (result) => {
          if (result.isSuccess()) {
            await this.router.navigateByUrl(this.routeStateService.getPreviousUrl());
          } else {
            this.errors = result.getResponse().error.errors;
          }
          this.loginInProgress = false;
        },
        (err) => {
          this.loginInProgress = false;
          console.log(err);
        },
      );
  }
}
