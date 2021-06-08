import { ComponentFixture, TestBed, waitForAsync } from '@angular/core/testing';

import { MarkdownFieldComponent } from './markdown-field.component';

describe('MarkdownFieldComponent', () => {
  let component: MarkdownFieldComponent;
  let fixture: ComponentFixture<MarkdownFieldComponent>;

  beforeEach(waitForAsync(() => {
    TestBed.configureTestingModule({
      declarations: [ MarkdownFieldComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(MarkdownFieldComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
