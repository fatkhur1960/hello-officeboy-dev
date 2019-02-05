
import React, { Component } from 'react';
import { Icon, Label, Menu, Table, Search, Input } from 'semantic-ui-react'

import Page from './Page';
import AnsTable from './AnsTable';

import './AccountPage.css';


export default class AccountPage extends Page {
  constructor(props) {
    super(props);
    this.id = "AccountPage";
    this.state = {
      title: "Accounts",
    }

    // this.loadPage(0, this.limit);

  }

  topbar() {
    return <Input icon="search" loading={this.state.search} focus placeholder="Search..."
      onKeyPress={(ev) => {
        if (ev.key == 'Enter') {
          this.setState({ search: true })
          this.page = 0;
          this.table.search(ev.target.value, this.page, 5, () => {
            this.setState({ search: false });
          });
        }
      }} />
  }

  content() {
    return <AnsTable endpoint="accounts" search-endpoint="account/search"
      columns={[
        "ID", "Name", "Email", "Phone", "Registered", "Active"
      ]} ref={table => this.table = table} />
  }
}
