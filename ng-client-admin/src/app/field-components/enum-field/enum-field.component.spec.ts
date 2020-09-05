import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { EnumFieldComponent } from './enum-field.component';

describe('EnumFieldComponent', () => {
  let component: EnumFieldComponent;
  let fixture: ComponentFixture<EnumFieldComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ EnumFieldComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(EnumFieldComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
