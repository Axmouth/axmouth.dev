import { Component, OnInit } from '@angular/core';
import { Title } from '@angular/platform-browser';

@Component({
  selector: 'app-contact-success-page',
  templateUrl: './contact-success-page.component.html',
  styleUrls: ['./contact-success-page.component.scss'],
})
export class ContactSuccessPageComponent implements OnInit {
  constructor(private title: Title) {}

  ngOnInit(): void {
    this.title.setTitle('axmouth.dev - Seccessfully Sent Email');
  }
}
