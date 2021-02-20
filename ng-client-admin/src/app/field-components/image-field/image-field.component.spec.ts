import { ComponentFixture, TestBed, waitForAsync } from '@angular/core/testing';

import { ImageFieldComponent } from './image-field.component';

describe('ImageFieldComponent', () => {
  let component: ImageFieldComponent;
  let fixture: ComponentFixture<ImageFieldComponent>;

  beforeEach(waitForAsync(() => {
    TestBed.configureTestingModule({
      declarations: [ ImageFieldComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(ImageFieldComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
