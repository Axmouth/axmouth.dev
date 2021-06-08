import { ComponentFixture, TestBed, waitForAsync } from '@angular/core/testing';

import { TitleFieldComponent } from './title-field.component';

describe('TitleFieldComponent', () => {
  let component: TitleFieldComponent;
  let fixture: ComponentFixture<TitleFieldComponent>;

  beforeEach(waitForAsync(() => {
    TestBed.configureTestingModule({
      declarations: [ TitleFieldComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(TitleFieldComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
