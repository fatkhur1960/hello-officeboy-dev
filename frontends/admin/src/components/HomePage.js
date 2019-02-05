
import React, { Component } from 'react';
import { Icon, Statistic } from 'semantic-ui-react'

import Page from './Page';
import apiClient from './axios';

// import './AccountPage.css';


export default class HomePage extends Page {
  constructor(props) {
    super(props);
    this.id = "HomePage";
    this.state = {
      title: "Statistics",
      userCount: 0
    }

    // this.loadPage(0, this.limit);

  }

  componentDidMount() {
    apiClient.private.get("account/count")
      .then((resp) => {
        this.setState({ userCount: resp.data.result })
      })
  }


  topbar() {

  }

  content() {
    return (
      <Statistic>
        <Statistic.Value>
          <Icon name='users' />
          {this.state.userCount}
        </Statistic.Value>
        <Statistic.Label>Accounts</Statistic.Label>
      </Statistic>
    )
  }
}
