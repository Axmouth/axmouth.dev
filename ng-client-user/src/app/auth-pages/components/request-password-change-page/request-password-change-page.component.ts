import { Component, OnInit } from '@angular/core';
import { Title } from '@angular/platform-browser';

@Component({
  selector: 'app-request-password-change-page',
  templateUrl: './request-password-change-page.component.html',
  styleUrls: ['./request-password-change-page.component.scss'],
})
export class RequestPasswordChangePageComponent implements OnInit {
  constructor(private title: Title) {}

  ngOnInit(): void {
    this.title.setTitle('axmouth.dev - Request Password Reset');
  }
}
