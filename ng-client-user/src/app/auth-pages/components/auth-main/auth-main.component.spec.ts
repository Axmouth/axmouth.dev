import { ComponentFixture, TestBed, waitForAsync } from '@angular/core/testing';

import { AuthMainComponent } from './auth-main.component';

describe('AuthMainComponent', () => {
  let component: AuthMainComponent;
  let fixture: ComponentFixture<AuthMainComponent>;

  beforeEach(waitForAsync(() => {
    TestBed.configureTestingModule({
      declarations: [ AuthMainComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(AuthMainComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
