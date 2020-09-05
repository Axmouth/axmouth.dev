import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { VerifyEmailPageComponent } from './verify-email-page.component';

describe('VerifyEmailPageComponent', () => {
  let component: VerifyEmailPageComponent;
  let fixture: ComponentFixture<VerifyEmailPageComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ VerifyEmailPageComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(VerifyEmailPageComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
