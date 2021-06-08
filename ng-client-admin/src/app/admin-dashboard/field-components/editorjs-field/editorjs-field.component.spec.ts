import { ComponentFixture, TestBed, waitForAsync } from '@angular/core/testing';

import { EditorjsFieldComponent } from './editorjs-field.component';

describe('EditorjsFieldComponent', () => {
  let component: EditorjsFieldComponent;
  let fixture: ComponentFixture<EditorjsFieldComponent>;

  beforeEach(waitForAsync(() => {
    TestBed.configureTestingModule({
      declarations: [ EditorjsFieldComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(EditorjsFieldComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
