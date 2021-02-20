import { ComponentFixture, TestBed, waitForAsync } from '@angular/core/testing';

import { ViewEntityComponent } from './view-entity.component';

describe('ViewEntityComponent', () => {
  let component: ViewEntityComponent;
  let fixture: ComponentFixture<ViewEntityComponent>;

  beforeEach(waitForAsync(() => {
    TestBed.configureTestingModule({
      declarations: [ ViewEntityComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(ViewEntityComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
