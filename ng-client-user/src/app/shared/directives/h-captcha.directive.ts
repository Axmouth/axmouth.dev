import {
  AfterViewInit,
  Directive,
  ElementRef,
  EventEmitter,
  forwardRef,
  Injector,
  Input,
  NgZone,
  OnInit,
  Output,
  Type,
} from '@angular/core';
import { ControlValueAccessor, FormControl, NG_VALUE_ACCESSOR, NgControl } from '@angular/forms';

// tslint:disable-next-line: class-name interface-name
export interface hCaptchaConfig {
  theme?: 'dark' | 'light';
  type?: 'audio' | 'image';
  size?: 'compact' | 'normal';
  tabindex?: number;
}

declare const hcaptcha: any;

declare global {
  // tslint:disable-next-line: interface-name
  interface Window {
    hcaptcha: any;
    hCaptchaLoad: () => void;
  }
}

@Directive({
  // tslint:disable-next-line:directive-selector
  selector: '[nbHCaptcha]',
  exportAs: 'nbRecaptcha',
  providers: [
    {
      provide: NG_VALUE_ACCESSOR,
      useExisting: forwardRef(() => HCaptchaDirective),
      multi: true,
    },
  ],
})
export class HCaptchaDirective implements OnInit, AfterViewInit, ControlValueAccessor {
  @Input() key: string;
  @Input() config: hCaptchaConfig;
  @Input() lang: string;

  @Output() captchaResponse = new EventEmitter<string>();
  @Output() captchaExpired = new EventEmitter();

  private control: FormControl;
  private onChange: (value: string) => void;
  private onTouched: (value: string) => void;
  private widgetId: number;
  loaded = false;

  /**
   * Add the script
   */
  addHCaptchaScript() {
    const script = document.createElement('script');
    const lang = this.lang ? '&hl=' + this.lang : '';
    script.src = `https://hcaptcha.com/1/api.js?onload=hCaptchaLoad&render=explicit${lang}`;
    script.async = true;
    script.defer = true;
    document.body.appendChild(script);
  }

  constructor(private element: ElementRef, private ngZone: NgZone, private injector: Injector) {}

  ngOnInit() {
    this.registerHCaptchaCallback();
    this.addHCaptchaScript();
    if (this.loaded === false) {
      window.hCaptchaLoad();
    }
  }

  ngAfterViewInit() {
    this.control = this.injector.get<NgControl>(NgControl).control as FormControl;
  }

  /**
   * Useful for multiple captcha
   */
  getId() {
    return this.widgetId;
  }

  registerHCaptchaCallback() {
    window.hCaptchaLoad = () => {
      const config = {
        ...this.config,
        sitekey: this.key,
        callback: this.onSuccess.bind(this),
        'expired-callback': this.onExpired.bind(this),
      };

      console.log(config);
      this.widgetId = this.render(this.element.nativeElement, config);
      this.loaded = true;
    };
  }

  /**
   * Renders the container as a reCAPTCHA widget and returns the ID of the newly created widget.
   */
  private render(element: HTMLElement, config: hCaptchaConfig): number {
    return hcaptcha.render(element, config);
  }

  /**
   * Resets the reCAPTCHA widget.
   */
  reset(): void {
    if (!this.widgetId) {
      return;
    }
    hcaptcha.reset(this.widgetId);
    this.onChange(null);
  }

  /**
   * Gets the response for the reCAPTCHA widget.
   */
  getResponse(): string {
    if (!this.widgetId) {
      return hcaptcha.getResponse(this.widgetId);
    }
  }

  writeValue(obj: any): void {}

  registerOnChange(fn: any): void {
    this.onChange = fn;
  }

  registerOnTouched(fn: any): void {
    this.onTouched = fn;
  }

  onExpired() {
    this.ngZone.run(() => {
      this.captchaExpired.emit();
      this.onChange(null);
      this.onTouched(null);
    });
  }

  onSuccess(token: string) {
    this.ngZone.run(() => {
      this.captchaResponse.next(token);
      this.onChange(token);
      this.onTouched(token);
    });
  }
}
