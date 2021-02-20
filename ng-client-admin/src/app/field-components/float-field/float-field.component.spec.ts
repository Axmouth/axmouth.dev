import { ComponentFixture, TestBed, waitForAsync } from '@angular/core/testing';

import { FloatFieldComponent } from './float-field.component';

describe('FloatFieldComponent', () => {
  let component: FloatFieldComponent;
  let fixture: ComponentFixture<FloatFieldComponent>;

  beforeEach(waitForAsync(() => {
    TestBed.configureTestingModule({
      declarations: [ FloatFieldComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(FloatFieldComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
