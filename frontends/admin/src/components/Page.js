
import React, { Component } from 'react';
import 'semantic-ui-css/semantic.min.css';


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
        {this.topbar()}
        {this.content()}
      </div>
    </div>)
  }

  topbar() {
    return null;
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
