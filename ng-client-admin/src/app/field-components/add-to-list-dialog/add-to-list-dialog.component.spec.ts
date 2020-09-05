import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { AddToListDialogComponent } from './add-to-list-dialog.component';

describe('AddToListDialogComponent', () => {
  let component: AddToListDialogComponent;
  let fixture: ComponentFixture<AddToListDialogComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ AddToListDialogComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(AddToListDialogComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
