import { ComponentFixture, TestBed, waitForAsync } from '@angular/core/testing';

import { CommentPostedComponent } from './comment-posted.component';

describe('CommentPostedComponent', () => {
  let component: CommentPostedComponent;
  let fixture: ComponentFixture<CommentPostedComponent>;

  beforeEach(waitForAsync(() => {
    TestBed.configureTestingModule({
      declarations: [ CommentPostedComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(CommentPostedComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
