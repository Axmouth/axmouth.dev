import { ComponentFixture, TestBed, waitForAsync } from '@angular/core/testing';

import { ContactMainComponent } from './contact-main.component';

describe('ContactMainComponent', () => {
  let component: ContactMainComponent;
  let fixture: ComponentFixture<ContactMainComponent>;

  beforeEach(waitForAsync(() => {
    TestBed.configureTestingModule({
      declarations: [ ContactMainComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(ContactMainComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
