import React, { Component } from 'react';
import { Icon, Label, Menu, Table, Search, Input } from 'semantic-ui-react'

import apiClient from './axios';

export default class AnsTable extends Component {
  constructor(props) {
    super(props);
    this.endpoint = props.endpoint;
    this.searchEndpoint = props["search-endpoint"]
    this.page = props.page ? props.page : 0;
    this.limit = props.limit ? props.limit : 5;
    this.state = {
      items: [],
      paging: []
    }
  }

  loadPage(page, limit) {
    this.page = page;
    apiClient.private.get(`/${this.endpoint}?page=${page}&limit=${limit}`)
      .then((resp) => this.handleResult(resp));
  }

  search(keyword, page, limit, cb) {
    var limit = limit ? limit : this.limit;
    this.page = page;
    this.setState({ search: true });
    apiClient.private.get(`/${this.searchEndpoint}?query=${keyword}&page=${page}&limit=${limit}`)
      .then((resp) => {
        this.handleResult(resp);
        cb();
      });
  }

  handleResult(resp) {
    var count = Math.round(resp.data.count / this.limit),
      remain = resp.data.count % this.limit > 0;
    var paging = Array.from(Array(count + 1).keys());
    this.setState({ items: resp.data.entries, paging: paging, search: false });
  }

  buildColumn() {
    return (
      <Table.Row>
        {
          this.props.columns.map((a) => <Table.HeaderCell key={a}>{a}</Table.HeaderCell>)
        }
      </Table.Row>
    )
  }

  buildRow(a) {
    return (
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
    )
  }

  componentDidMount() {
    this.loadPage(this.page, this.limit);
  }

  render() {
    return (
      <Table celled>
        <Table.Header>
          {this.buildColumn()}
        </Table.Header>

        <Table.Body>
          {
            this.state.items.map((a) => (
              this.buildRow(a)
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
    )
  }
}


