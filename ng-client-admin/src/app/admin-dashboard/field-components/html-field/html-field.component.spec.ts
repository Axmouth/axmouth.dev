import { ComponentFixture, TestBed, waitForAsync } from '@angular/core/testing';

import { HtmlFieldComponent } from './html-field.component';

describe('HtmlFieldComponent', () => {
  let component: HtmlFieldComponent;
  let fixture: ComponentFixture<HtmlFieldComponent>;

  beforeEach(waitForAsync(() => {
    TestBed.configureTestingModule({
      declarations: [ HtmlFieldComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(HtmlFieldComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
