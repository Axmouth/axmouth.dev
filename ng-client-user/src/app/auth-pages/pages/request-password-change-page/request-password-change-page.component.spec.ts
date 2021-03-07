import { ComponentFixture, TestBed, waitForAsync } from '@angular/core/testing';

import { RequestPasswordChangePageComponent } from './request-password-change-page.component';

describe('RequestPasswordChangePageComponent', () => {
  let component: RequestPasswordChangePageComponent;
  let fixture: ComponentFixture<RequestPasswordChangePageComponent>;

  beforeEach(waitForAsync(() => {
    TestBed.configureTestingModule({
      declarations: [ RequestPasswordChangePageComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(RequestPasswordChangePageComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
