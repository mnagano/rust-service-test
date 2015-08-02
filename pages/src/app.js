var React = require('react/addons');
var Router = require('director').Router;
var NameResult = require('./name_result.js');
var CountResult = require('./count_result.js');
var NameListResult = require('./name_list_result.js');


var App = React.createClass({
    getInitialState: function() {
        return {
            page: 'name'
        };
    },
    componentDidMount: function() {
        var setNamePage = function() {
            this.setState({ page: 'name'});
        }.bind(this);
        var setCountPage = function() {
            this.setState({ page: 'count' });
        }.bind(this);
        var setNameListPage = function() {
            this.setState({ page: 'names' });
        }.bind(this);
        var router = Router({
            '/name': setNamePage,
            '/count': setCountPage,
            '/names': setNameListPage,
            '*': setNamePage,
        });
        router.init();
    },
    render: function () {
        var page;
        if (this.state.page === 'name'){
            page = <NameResult/>;
        } else if (this.state.page === 'count'){
            page = <CountResult/>
        }else{
            page = <NameListResult/>
        }

        return (
            <div>
                <h1>赤ちゃんの名前をつけよう</h1>
                <ul>
                    <li><a href="#/name">名前で調べてみよう</a></li>
                    <li><a href="#/count">よい画数を調べてみよう</a></li>
                    <li><a href="#/names">画数で検索してみよう</a></li>
                </ul>
                {page}
            </div>
        );
    }
});

React.render(
    <App/>, document.getElementById('app-container')
);