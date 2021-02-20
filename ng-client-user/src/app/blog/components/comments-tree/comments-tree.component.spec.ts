import { ComponentFixture, TestBed, waitForAsync } from '@angular/core/testing';

import { CommentsTreeComponent } from './comments-tree.component';

describe('CommentsTreeComponent', () => {
  let component: CommentsTreeComponent;
  let fixture: ComponentFixture<CommentsTreeComponent>;

  beforeEach(waitForAsync(() => {
    TestBed.configureTestingModule({
      declarations: [ CommentsTreeComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(CommentsTreeComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
