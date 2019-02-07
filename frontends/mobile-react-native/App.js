
import React, { Component } from 'react';
import { AppRegistry, FlatList, StyleSheet, Text, View, Alert, Button } from 'react-native';
import Login from './components/Login';

function ApiClient(baseUrl) {
  this.baseUrl = baseUrl;
  this.get = function (path) {
    return fetch(this.baseUrl + path);
  }
  this.post = function(path, data){
    return fetch(path, {
      method: 'POST',
      headers: {
        Accept: 'application/json',
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(data)
    });
  }
  return this;
}

const api = {
  public: ApiClient("http://172.20.10.2:8081/api/payment/v1"),
  private: ApiClient("http://172.20.10.2:8082/api/payment/v1")
};

// class FlatListBasics extends Component {

//   constructor(props) {
//     super(props);
//     this.state = {
//       entries: []
//     }
//     api.private.get("/accounts?page=0&limit=10")
//       .then((resp) => resp.json())
//       .then((data) => {
//         this.setState({
//           isLoading: false,
//           data: data.entries
//         })
//       })
//   }

//   onItemPress(fullName) {
//     Alert.alert("you clicked: " + fullName);
//   }

//   render() {
//     return (
//       <View style={styles2.container}>
//         <FlatList
//           data={this.state.data}
//           renderItem={({ item }) => <Text style={styles2.item} onPress={() => this.onItemPress(item.full_name)}>{item.full_name}</Text>}
//           keyExtractor={(item, index) => index.toString()}
//         />
//       </View>
//     );
//   }
// }

// const styles2 = StyleSheet.create({
//   container: {
//     flex: 1,
//     paddingTop: 22
//   },
//   item: {
//     padding: 10,
//     fontSize: 18,
//     height: 44,
//   },
// })



export default class App extends React.Component {
  render() {
    return (
      <View style={styles.container}>
        
        <Login />

      </View>
    );
  }
}

const styles = StyleSheet.create({
  container: {
      flex: 1,
      paddingTop: 100,
      backgroundColor: '#2c3e50',
  }
})