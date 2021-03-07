import { ComponentFixture, TestBed, waitForAsync } from '@angular/core/testing';

import { ContactSuccessPageComponent } from './contact-success-page.component';

describe('ContactSuccessPageComponent', () => {
  let component: ContactSuccessPageComponent;
  let fixture: ComponentFixture<ContactSuccessPageComponent>;

  beforeEach(waitForAsync(() => {
    TestBed.configureTestingModule({
      declarations: [ ContactSuccessPageComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(ContactSuccessPageComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
