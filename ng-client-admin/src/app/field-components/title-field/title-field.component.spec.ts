import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { TitleFieldComponent } from './title-field.component';

describe('TitleFieldComponent', () => {
  let component: TitleFieldComponent;
  let fixture: ComponentFixture<TitleFieldComponent>;

  beforeEach(async(() => {
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
