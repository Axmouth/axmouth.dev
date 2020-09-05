import { Component, OnInit } from '@angular/core';
import { FormGroup, FormControl } from '@angular/forms';

@Component({
  selector: 'app-blog-search-side-widget',
  templateUrl: './blog-search-side-widget.component.html',
  styleUrls: ['./blog-search-side-widget.component.scss'],
})
export class BlogSearchSideWidgetComponent implements OnInit {
  searchForm = new FormGroup({
    searchQ: new FormControl(''),
  });

  constructor() {}

  ngOnInit(): void {}
}
