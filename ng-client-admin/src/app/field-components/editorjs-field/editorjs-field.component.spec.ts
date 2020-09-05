import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { EditorjsFieldComponent } from './editorjs-field.component';

describe('EditorjsFieldComponent', () => {
  let component: EditorjsFieldComponent;
  let fixture: ComponentFixture<EditorjsFieldComponent>;

  beforeEach(async(() => {
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
