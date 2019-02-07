import React, { Component } from 'react';
import { View, Text, TextInput, TouchableOpacity, StyleSheet } from 'react-native';
import crypto from '../libs/apf-crypto';

export default class LoginForm extends Component {
  constructor(props) {
    super(props);
    this.state = {
    };
  }

  onButtonPress(event){
    // console.log(event)
    console.log("user name: " + this.state.userName);
    console.log("password: " + this.state.password);
    let passhash = crypto.getPasshash(this.state.password);
    console.log("passhash: " + passhash);
  }

  render() {
    return (
      <View style={styles.container}>
        <TextInput style={styles.input}
          autoCapitalize="none"
          onSubmitEditing={() => this.passwordInput.focus()}
          autoCorrect={false}
          keyboardType='email-address'
          returnKeyType="next"
          placeholder='Email or Mobile Number'
          placeholderTextColor='rgba(225,225,225,0.7)' 
          onChangeText={(text) => this.setState({userName: text})} />

        <TextInput style={styles.input}
          returnKeyType="go"
          ref={(input) => this.passwordInput = input}
          placeholder='Password'
          placeholderTextColor='rgba(225,225,225,0.7)'
          secureTextEntry 
          onChangeText={(text) => this.setState({password: text})}
          onSubmitEditing={(ev) => this.onButtonPress(ev)} />

        <TouchableOpacity style={styles.buttonContainer}
          onPress={(ev)=> this.onButtonPress(ev)}>
          <Text style={styles.buttonText}>LOGIN</Text>
        </TouchableOpacity>
      </View>
    );
  }
}

// define your styles
const styles = StyleSheet.create({
  container: {
   padding: 20,
   marginTop: 100
  },
  input:{
      height: 40,
      backgroundColor: 'rgba(225,225,225,0.2)',
      marginBottom: 10,
      padding: 10,
      color: '#fff'
  },
  buttonContainer:{
      backgroundColor: '#2980b6',
      paddingVertical: 15
  },
  buttonText:{
      color: '#fff',
      textAlign: 'center',
      fontWeight: '700'
  }
})