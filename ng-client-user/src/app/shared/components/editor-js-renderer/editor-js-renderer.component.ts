import { isPlatformServer } from '@angular/common';
import { Component, OnInit, Input, Inject, PLATFORM_ID } from '@angular/core';

@Component({
  selector: 'app-editor-js-renderer',
  templateUrl: './editor-js-renderer.component.html',
  styleUrls: ['./editor-js-renderer.component.scss'],
})
export class EditorJsRendererComponent implements OnInit {
  @Input()
  content: any;
  ssr = false;

  constructor(@Inject(PLATFORM_ID) private platform: object) {
    if (isPlatformServer(this.platform)) {
      this.ssr = true;
    }
  }

  ngOnInit(): void {}

  zoomInOut(event: Event) {}

  isImage(url: string) {
    return true;
  }
}
