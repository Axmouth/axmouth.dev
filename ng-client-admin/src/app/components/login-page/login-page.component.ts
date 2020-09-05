import { Component, OnInit } from '@angular/core';
import { FormGroup, FormBuilder, Validators } from '@angular/forms';
import { ActivatedRoute, Router } from '@angular/router';
import { AuthService } from 'src/auth';

@Component({
  selector: 'app-login-page',
  templateUrl: './login-page.component.html',
  styleUrls: ['./login-page.component.scss'],
})
export class LoginPageComponent implements OnInit {
  form: FormGroup;
  public loginInvalid: boolean;
  private formSubmitAttempt: boolean;
  private returnUrl: string;
  errors = [];

  constructor(
    private fb: FormBuilder,
    private route: ActivatedRoute,
    private router: Router,
    private authService: AuthService,
  ) {}

  async ngOnInit(): Promise<void> {
    this.returnUrl = this.route.snapshot.queryParams.returnUrl || '/game';
    this.form = this.fb.group({
      username: ['', Validators.email],
      password: ['', Validators.required],
    });
    if (await this.authService.isAuthenticated().toPromise()) {
      await this.router.navigate([this.returnUrl]);
    }
  }

  async onSubmit() {
    this.loginInvalid = false;
    this.formSubmitAttempt = false;
    if (this.form.valid) {
      const username = this.form.get('username').value;
      const password = this.form.get('password').value;
      const result = await this.authService.authenticate({ email: username, password }).toPromise();
      if (result.isFailure()) {
        this.loginInvalid = true;
        this.errors = result.getResponse().error.errors;
      } else {
        this.router.navigate(['/']);
      }
    } else {
      this.formSubmitAttempt = true;
    }
  }
}
