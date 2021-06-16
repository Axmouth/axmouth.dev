import { Component, OnInit } from '@angular/core';
import { FormGroup, FormControl } from '@angular/forms';
import { Router } from '@angular/router';

@Component({
  selector: 'app-search-side-widget',
  templateUrl: './search-side-widget.component.html',
  styleUrls: ['./search-side-widget.component.scss'],
})
export class SearchSideWidgetComponent implements OnInit {
  searchForm = new FormGroup({
    searchText: new FormControl(''),
  });

  constructor(private router: Router) {}

  ngOnInit(): void {}

  onSearchSubmit() {
    this.router.navigate(['/search', this.searchForm.get('searchText').value]);
  }
}
