import React, { Component } from 'react';
import logo from './logo.svg';
import './App.css';

import '@trendmicro/react-sidenav/dist/react-sidenav.css';
import SideNav, { Toggle, Nav, NavItem, NavIcon, NavText } from '@trendmicro/react-sidenav';

import { BrowserRouter as Router, Route, Link } from "react-router-dom";
import 'font-awesome/css/font-awesome.min.css';

import Page, { Index } from './components/Page';
import AccountPage from './components/AccountPage';
import HomePage from './components/HomePage';


class App extends Component {
  constructor() {
    super();
    this.state = {
      expanded: false
    };
  }

  render() {
    return (
      <div className="App">
        <header className="App-header">

          <Router>
            <Route render={({ location, history }) => (
              <React.Fragment>

                <SideNav id="SideNav"
                  expanded={this.state.expanded}
                  onToggle={(expanded) => {
                    this.setState({ expanded })
                  }}
                  onSelect={(selected) => {
                    const to = '/' + selected;
                    if (location.pathname !== to) {
                      history.push(to);
                    }
                  }}
                >
                  <SideNav.Toggle />
                  <SideNav.Nav>
                    <NavItem eventKey="home">
                      <NavIcon>
                        <i className="fa fa-fw fa-dashboard" style={{ fontSize: '1.75em' }} />
                      </NavIcon>
                      <NavText>
                        Dashboard
                        </NavText>
                    </NavItem>
                    <NavItem eventKey="account">
                      <NavIcon>
                        <i className="fa fa-fw fa-users" style={{ fontSize: '1.75em' }} />
                      </NavIcon>
                      <NavText>
                        Account
                        </NavText>
                    </NavItem>
                    <NavItem eventKey="about">
                      <NavIcon>
                        <i className="fa fa-fw fa-info" style={{ fontSize: '1.75em' }} />
                      </NavIcon>
                      <NavText>
                        About
                        </NavText>
                    </NavItem>
                  </SideNav.Nav>
                </SideNav>

                <main>
                  {/* <Route path="/" exact component={props => <RootComponent />} /> */}
                  <Route path="/home" component={props => <HomePage title="Dashboard" expanded={this.state.expanded} />} />
                  <Route path="/account" component={props => <AccountPage expanded={this.state.expanded} />} />
                  <Route path="/about" component={props => <Page title="About" expanded={this.state.expanded} />} />
                </main>
              </React.Fragment>
            )}
            />
          </Router>

        </header>
      </div>
    );
  }
}

export default App;
