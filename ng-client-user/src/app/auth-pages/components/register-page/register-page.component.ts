import { Component, OnInit } from '@angular/core';
import { FormGroup, FormControl, Validators, AbstractControl, ValidatorFn } from '@angular/forms';
import { AuthService } from 'src/auth';
import { Router } from '@angular/router';
import { RouteStateService } from 'src/app/shared/services/route-state.service';
import { CustomValidators } from 'src/app/shared/helpers/custom-validators';

export class MismatchValidator {
  static mismatch(otherInputControl: AbstractControl): ValidatorFn {
    return (inputControl: AbstractControl): { [key: string]: boolean } | null => {
      if (
        inputControl.value !== undefined &&
        inputControl.value.trim() !== '' &&
        inputControl.value !== otherInputControl.value
      ) {
        return { mismatch: true };
      }

      return null;
    };
  }
}

@Component({
  selector: 'app-register-page',
  templateUrl: './register-page.component.html',
  styleUrls: ['./register-page.component.scss'],
})
export class RegisterPageComponent implements OnInit {
  registerForm = new FormGroup({
    email: new FormControl('', [Validators.required, Validators.email]),
    displayName: new FormControl('', [
      Validators.required,
      Validators.minLength(3),
      Validators.maxLength(25),
      // check whether the entered name has no special character
      CustomValidators.patternValidator(/^[^!@#$%^&*()_+\-=\[\]{};':"\\|,.<>\/?]+$/, {
        hasNoSpecialCharacters: true,
      }),
      // check whether the entered name starts or ends with a space
      CustomValidators.patternValidator(/^[^ ]+.*[^ ]+$/, {
        hasSpacePrefixOrSuffix: true,
      }),
    ]),
    password: new FormControl(''),
    password2: new FormControl(''),
  });
  errors = [];

  registerInProgress = false;

  constructor(private authService: AuthService, private router: Router, private routeStateService: RouteStateService) {}

  ngOnInit(): void {
    this.setValidators();
  }

  private setValidators() {
    const formValidators = {
      password: Validators.compose([
        Validators.required,
        Validators.minLength(6),
        Validators.maxLength(35),
        // check whether the entered password has a number
        CustomValidators.patternValidator(/\d/, { hasNumber: true }),
        // check whether the entered password has upper case letter
        CustomValidators.patternValidator(/[A-Z]/, { hasUpperCase: true }),
        // check whether the entered password has a lower-case letter
        CustomValidators.patternValidator(/[a-z]/, { hasLowerCase: true }),
        // check whether the entered password has a special character
        CustomValidators.patternValidator(/[ !@#$%^&*()_+\-=\[\]{};':"\\|,.<>\/?]/, { hasSpecialCharacters: true }),
      ]),
      password2: Validators.compose([
        Validators.required,
        MismatchValidator.mismatch(this.registerForm.get('password')),
      ]),
    };

    this.registerForm.get('password').setValidators(formValidators.password);
    this.registerForm.get('password2').setValidators(formValidators.password2);
  }

  onSubmit() {
    this.registerInProgress = true;
    this.authService
      .register({
        email: this.registerForm.get('email').value,
        password: this.registerForm.get('password').value,
        displayName: this.registerForm.get('displayName').value,
      })
      .subscribe(
        async (result) => {
          if (result.isSuccess()) {
            await this.router.navigateByUrl(this.routeStateService.getPreviousUrl());
          } else {
            this.errors = result.getResponse().error.errors;
          }
          this.registerInProgress = false;
        },
        (err) => {
          this.registerInProgress = false;
          console.log(err);
        },
      );
  }
}
