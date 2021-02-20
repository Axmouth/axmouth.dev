import { ComponentFixture, TestBed, waitForAsync } from '@angular/core/testing';

import { FieldChooserComponent } from './field-chooser.component';

describe('FieldChooserComponent', () => {
  let component: FieldChooserComponent;
  let fixture: ComponentFixture<FieldChooserComponent>;

  beforeEach(waitForAsync(() => {
    TestBed.configureTestingModule({
      declarations: [ FieldChooserComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(FieldChooserComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
