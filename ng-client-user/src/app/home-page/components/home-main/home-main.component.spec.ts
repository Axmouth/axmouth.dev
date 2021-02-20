import { ComponentFixture, TestBed, waitForAsync } from '@angular/core/testing';

import { HomeMainComponent } from './home-main.component';

describe('HomeMainComponent', () => {
  let component: HomeMainComponent;
  let fixture: ComponentFixture<HomeMainComponent>;

  beforeEach(waitForAsync(() => {
    TestBed.configureTestingModule({
      declarations: [ HomeMainComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(HomeMainComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
