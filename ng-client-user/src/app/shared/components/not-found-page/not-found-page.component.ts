import { DOCUMENT } from '@angular/common';
import { Component, Inject, OnInit } from '@angular/core';
import { Meta, Title } from '@angular/platform-browser';
import { websiteUrl } from 'src/environments/environment';

@Component({
  selector: 'app-not-found-page',
  templateUrl: './not-found-page.component.html',
  styleUrls: ['./not-found-page.component.scss'],
})
export class NotFoundPageComponent implements OnInit {
  constructor(private title: Title, private meta: Meta, @Inject(DOCUMENT) private doc: Document) {}

  ngOnInit(): void {
    this.title.setTitle(`Page Not Found | Axmouth's Website`);
    this.meta.updateTag({ name: `title`, content: this.title.getTitle() });
    this.meta.updateTag({
      property: `og:url`,
      content: this.doc.location.href.replace(this.doc.location.origin, websiteUrl),
    });
    this.meta.updateTag({ property: `og:title`, content: this.title.getTitle() });
    this.meta.updateTag({
      property: `twitter:url`,
      content: this.doc.location.href.replace(this.doc.location.origin, websiteUrl),
    });
    this.meta.updateTag({ property: `twitter:title`, content: this.title.getTitle() });
  }
}
