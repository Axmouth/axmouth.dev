import { ComponentFixture, TestBed, waitForAsync } from '@angular/core/testing';

import { CommentPreviewPageComponent } from './comment-preview-page.component';

describe('CommentPreviewPageComponent', () => {
  let component: CommentPreviewPageComponent;
  let fixture: ComponentFixture<CommentPreviewPageComponent>;

  beforeEach(waitForAsync(() => {
    TestBed.configureTestingModule({
      declarations: [ CommentPreviewPageComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(CommentPreviewPageComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
