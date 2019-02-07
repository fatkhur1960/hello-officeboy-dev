import React, { Component } from 'react';
import { View, Text, StyleSheet, KeyboardAvoidingView, Image } from 'react-native';
import LoginForm from './LoginForm';

export default class Login extends Component {
  constructor(props) {
    super(props);
    this.state = {
    };
  }

  render() {
    return (
      <KeyboardAvoidingView behavior="padding" style={styles.container} keyboardVerticalOffset={400}>

        <View style={styles.loginContainer}>
          <Image resizeMode="contain" style={styles.logo} source={require('../components/images/retro-coin-icon.png')} />
        </View>


        <View>
          <Text style={styles.welcomeText}>Ansvia Payment Framework</Text>
        </View>

        <View style={styles.formContainer}>
          <LoginForm />
        </View>
      </KeyboardAvoidingView>
    );
  }
}

// define your styles
const styles = StyleSheet.create({
  container: {
    flex: 1,
    // paddingTop: 100,
    // backgroundColor: '#2c3e50'
  },
  loginContainer: {
    alignItems: 'center',
    flexGrow: 1,
    justifyContent: 'center'
  },
  logo: {
    // position: 'absolute',
    paddingBottom: 10,
    width: 300,
    height: 100
  },
  welcomeText: {
    // marginTop: 50,
    fontWeight: 'bold',
    fontSize: 20,
    color: '#FFFFFF',
    textAlign: 'center'
  }
})