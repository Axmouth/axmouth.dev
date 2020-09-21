import { DOCUMENT } from '@angular/common';
import { Component, Inject, OnInit } from '@angular/core';
import { Meta, Title } from '@angular/platform-browser';

@Component({
  selector: 'app-verify-email-page',
  templateUrl: './verify-email-page.component.html',
  styleUrls: ['./verify-email-page.component.scss'],
})
export class VerifyEmailPageComponent implements OnInit {
  constructor(private title: Title, private meta: Meta, @Inject(DOCUMENT) private doc: Document) {}

  ngOnInit(): void {
    this.title.setTitle(`Verify Email - Axmouth's Website`);
    this.meta.updateTag({ name: `title`, content: this.title.getTitle() });
    this.meta.updateTag({ property: `og:url`, content: this.doc.location.href });
    this.meta.updateTag({ property: `og:title`, content: this.title.getTitle() });
    this.meta.updateTag({ property: `twitter:url`, content: this.doc.location.href });
    this.meta.updateTag({ property: `twitter:title`, content: this.title.getTitle() });
  }
}
