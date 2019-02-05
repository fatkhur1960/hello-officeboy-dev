
import React, { Component } from 'react';
// import MaterialTable from 'material-table'
import { Icon, Label, Menu, Table } from 'semantic-ui-react'
import 'semantic-ui-css/semantic.min.css';
import './AccountPage.css';

import apiClient from './axios';

export default class Page extends Component {
  constructor(props) {
    super(props);
    this.state = {
      title: props.title
    }
  }

  render() {
    let marginLeft = 5;
    if (this.props.expanded) {
      marginLeft = 170;
    }
    return (<div id={this.id} style={{ marginLeft: marginLeft, padding: 10 }}>
      <h2>{this.state.title}</h2>
      <div>
        {this.content()}
      </div>
    </div>)
  }

  content() {
    return null;
  }

  componentDidMount() {
    console.log("mounting...");
  }

  componentWillUnmount() {
    console.log("unmounting...");
  }
}

export class Index extends Page {
  constructor(props) {
    super(props);
  }
  render() {
    return (
      <div>
        {super.render()}
      </div>
    )
  }
}

export class AccountPage extends Page {
  constructor(props) {
    super(props);
    this.id = "AccountPage";
    this.limit = 5;
    this.state = {
      title: "Accounts",
      items: [],
      paging: []
    }

    this.loadPage(0, this.limit);

  }

  loadPage(offset, limit) {
    apiClient.private.get(`/accounts?page=${offset}&limit=${limit}`)
      .then((resp) => {
        var count = Math.round(resp.data.count / this.limit),
          remain = resp.data.count % this.limit > 0;
        var paging = Array.from(Array(count + 1).keys());
        this.setState({ items: resp.data.entries, paging: paging });
      });
  }

  content() {
    return <Table celled>
      <Table.Header>
        <Table.Row>
          <Table.HeaderCell>ID</Table.HeaderCell>
          <Table.HeaderCell>Name</Table.HeaderCell>
          <Table.HeaderCell>Email</Table.HeaderCell>
          <Table.HeaderCell>Phone</Table.HeaderCell>
          <Table.HeaderCell>Registered</Table.HeaderCell>
          <Table.HeaderCell>Active</Table.HeaderCell>
        </Table.Row>
      </Table.Header>

      <Table.Body>
        {
          this.state.items.map((a) => (
            <Table.Row key={a.id}>
              <Table.Cell>
                {a.id}
              </Table.Cell>
              <Table.Cell>{a.full_name}</Table.Cell>
              <Table.Cell>{a.email}</Table.Cell>
              <Table.Cell>{a.phone_num}</Table.Cell>
              <Table.Cell>{a.register_time}</Table.Cell>
              <Table.Cell>{a.active ? "ACTIVE" : "INACTIVE"}</Table.Cell>
            </Table.Row>
          ))
        }
      </Table.Body>

      <Table.Footer>
        <Table.Row>
          <Table.HeaderCell colSpan='6'>
            <Menu floated='right' pagination>
              <Menu.Item as='a' icon>
                <Icon name='chevron left' />
              </Menu.Item>
              {
                this.state.paging.map((i) =>
                  <Menu.Item as='a' key={i + 1} onClick={() => this.loadPage(i, this.limit)}>{i + 1}</Menu.Item>
                )
              }
              <Menu.Item as='a' icon>
                <Icon name='chevron right' />
              </Menu.Item>
            </Menu>
          </Table.HeaderCell>
        </Table.Row>
      </Table.Footer>
    </Table>
  }
}
