import { DOCUMENT } from '@angular/common';
import { Inject } from '@angular/core';
import { Component, OnInit } from '@angular/core';
import { Meta, Title } from '@angular/platform-browser';

@Component({
  selector: 'app-password-change-page',
  templateUrl: './password-change-page.component.html',
  styleUrls: ['./password-change-page.component.scss'],
})
export class PasswordChangePageComponent implements OnInit {
  constructor(private title: Title, private meta: Meta, @Inject(DOCUMENT) private doc: Document) {}

  ngOnInit(): void {
    this.title.setTitle(`Change Password - Axmouth's Website`);
    this.meta.updateTag({ name: `title`, content: this.title.getTitle() });
    this.meta.updateTag({ property: `og:url`, content: this.doc.location.href });
    this.meta.updateTag({ property: `og:title`, content: this.title.getTitle() });
    this.meta.updateTag({ property: `twitter:url`, content: this.doc.location.href });
    this.meta.updateTag({ property: `twitter:title`, content: this.title.getTitle() });
  }
}
