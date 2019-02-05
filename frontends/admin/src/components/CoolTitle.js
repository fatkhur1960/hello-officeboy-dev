
import React, {Component} from 'react';

const apiClient = require('./axios');


export default class CoolTitle extends Component {
    constructor(props) {
        super(props);
        this.state = {
            title: props.title,
            version: ""
        };
        apiClient.public.get("/info")
            .then((resp) => {
                console.log(resp);
                this.setState({
                    title: props.title,
                    version: resp.data.version
                });
            });
    }
    handleClick() {
        if (this.state.title === "Zufar Faruq Marufi") {
            this.setState({
                title: "Akmalana Kamilan Marufi!"
            });
        } else {
            this.setState({
                title: "Zufar Faruq Marufi"
            });
        }
    }
    render() {
        return (
            <div>
                <h1 onClick={() => this.handleClick()}>{this.state.title}</h1>
                <div>
                    <p>{this.state.version}</p>
                </div>
            </div>
        )
    }
}

