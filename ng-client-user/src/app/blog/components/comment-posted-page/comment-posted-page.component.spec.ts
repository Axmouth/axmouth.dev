import { ComponentFixture, TestBed, waitForAsync } from '@angular/core/testing';

import { CommentPostedPageComponent } from './comment-posted-page.component';

describe('CommentPostedPageComponent', () => {
  let component: CommentPostedPageComponent;
  let fixture: ComponentFixture<CommentPostedPageComponent>;

  beforeEach(waitForAsync(() => {
    TestBed.configureTestingModule({
      declarations: [ CommentPostedPageComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(CommentPostedPageComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
