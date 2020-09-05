import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { NameListFieldComponent } from './name-list-field.component';

describe('NameListFieldComponent', () => {
  let component: NameListFieldComponent;
  let fixture: ComponentFixture<NameListFieldComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ NameListFieldComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(NameListFieldComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
