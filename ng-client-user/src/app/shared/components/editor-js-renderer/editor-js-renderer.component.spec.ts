import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { EditorJsRendererComponent } from './editor-js-renderer.component';

describe('EditorJsRendererComponent', () => {
  let component: EditorJsRendererComponent;
  let fixture: ComponentFixture<EditorJsRendererComponent>;

  beforeEach(async(() => {
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
