import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { RequestPasswordChangePageComponent } from './request-password-change-page.component';

describe('RequestPasswordChangePageComponent', () => {
  let component: RequestPasswordChangePageComponent;
  let fixture: ComponentFixture<RequestPasswordChangePageComponent>;

  beforeEach(async(() => {
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
