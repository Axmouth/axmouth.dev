import { Component, Input, OnInit } from '@angular/core';
import { SearchItem } from 'src/app/models/api/search-item';

@Component({
  selector: 'app-search-result-preview',
  templateUrl: './search-result-preview.component.html',
  styleUrls: ['./search-result-preview.component.scss'],
})
export class SearchResultPreviewComponent implements OnInit {
  @Input()
  searchItem: SearchItem;

  typeLabels = {
    Project: 'Project',
    BlogPost: 'Blog Post',
    Page: '',
    ExternalLink: 'External Link',
  };

  constructor() {}

  ngOnInit(): void {}
}
