import { Component, OnInit, Input } from '@angular/core';

@Component({
  selector: 'app-editor-js-renderer',
  templateUrl: './editor-js-renderer.component.html',
  styleUrls: ['./editor-js-renderer.component.scss'],
})
export class EditorJsRendererComponent implements OnInit {
  @Input()
  content: any;

  constructor() {}

  ngOnInit(): void {}

  zoomInOut(event: Event) {}

  isImage(url: string) {
    return true;
  }
}
