import { ComponentFixture, TestBed, waitForAsync } from '@angular/core/testing';

import { EditorJsRendererComponent } from './editor-js-renderer.component';

describe('EditorJsRendererComponent', () => {
  let component: EditorJsRendererComponent;
  let fixture: ComponentFixture<EditorJsRendererComponent>;

  beforeEach(waitForAsync(() => {
    TestBed.configureTestingModule({
      declarations: [ EditorJsRendererComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(EditorJsRendererComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
