var React = require('react/addons');
var Loader = require('react-loader');
var TextInput = require('./name_input.js').textInput;
var RadioButton = require('./name_input.js').radioButton;

var CountForm = React.createClass({
    getInitialState: function () {
        return {
            lastName: '長野',
            lastNameInputStatus: true,
            isMaleIndex: 0,
            isNewCountIndex: 0
        };
    },
    handleLastNameChange: function (e, validationResult) {
        this.setState({
            lastName: e.target.value.trim(),
            lastNameInputStatus: validationResult,
        });
        this.props.onChange();
    },
    handleIsMaleChange: function (e, index) {
        this.setState({
            isMaleIndex: index
        });
    },
    handleIsNewCountChange: function (e, index) {
        this.setState({
            isNewCountIndex: index
        });
    },
    handleSubmit: function (e) {
        e.preventDefault();
        var lastName = this.state.lastName.trim();
        var isMail = this.state.isMaleIndex == 0;
        var isNewCount = this.state.isNewCountIndex != 0;
        this.props.onFormSubmit(lastName, isMail, isNewCount);
    },
    render: function () {
        var disabled = !this.state.lastNameInputStatus;

        var errorText = this.props.isError ?
            "指定された名前には対応していません。おそらく名前に使えない字と考えられます。" : null;

        return (
            <form className="form-horizontal" onSubmit={this.handleSubmit}>
                <p className="text-danger bg-danger">{errorText}</p>
                <TextInput id="lastName" name="姓 : " placeholder="姓" reg={"^[^ -~｡-ﾟ]+$"} minLen={1} maxLen={5}
                           errorText="1から5文字以内の漢字で指定してください"
                           onChange={this.handleLastNameChange}>{this.state.lastName}</TextInput>

                <RadioButton id="isMaleRadios" name="性別: " buttons={["男", "女"]} currentIndex={this.state.isMaleIndex}
                             onChange={this.handleIsMaleChange}/>

                <RadioButton id="isNewCountRadios" name="字体: " buttons={["旧字体", "新字体"]}
                             currentIndex={this.state.isNewCountIndex}
                             onChange={this.handleIsNewCountChange}/>

                <div className="form-group">
                    <div className="col-sm-offset-2 col-sm-10">
                        <button type="submit" className="btn btn-primary" disabled={disabled}>送る</button>
                    </div>
                </div>
            </form>
        );
    }
});

var CountResultItem = React.createClass({
    render: function () {
        var styles = {
            color: "#ff7f7f",
            backgroundColor: "#bfff7f",
            fontSize: '16px',
            fontWeight: 'bold'
        }
        var className;
        var totalPoint = this.props.totalPoint;
        if (totalPoint > 80) {
            className = "success"
        } else if (totalPoint > 60) {
            className = "warning";
        } else if (totalPoint < 40) {
            className = "danger";
        }

        return (
            <tr className={className}>
                <td style={styles}>{this.props.tiUn}</td>
                <td>{this.props.firstNameCounts.length.toString()}</td>
                <td>{this.props.firstNameCounts.toString()}</td>
                <td>{this.props.tenUn}</td>
                <td>{this.props.jinUn}</td>
                <td>{this.props.gaiUn}</td>
                <td>{this.props.souUn}</td>
                <td>{this.props.tenUnPoint}</td>
                <td>{this.props.tiUnPoint}</td>
                <td>{this.props.jinUnPoint}</td>
                <td>{this.props.gaiUnPoint}</td>
                <td>{this.props.souUnPoint}</td>
                <td>{this.props.inyouPoint}</td>
                <td style={styles}>{this.props.totalPoint.toString() + "点"}</td>
            </tr>
        );
    }
});

var CountResult = React.createClass({
    getInitialState: function () {
        return {
            results: [],
            loaded: true
        };
    },

    handleChange: function () {
        this.setState({
            isError: false
        });
    },

    handleFormSubmit: function (lastName, isMale, isNewCount) {
        this.setState({
            loaded: false
        });
        var url = '/api/names?'
            + 'last_name=' + lastName
            + '&is_male=' + isMale
            + '&is_new_count=' + isNewCount;

        //console.log(url);
        $.ajax({
            url: url,
            dataType: 'json',
            cache: false,
            success: function (data) {
                if (!data) {
                    this.setState({
                        isError: true,
                        loaded: true
                    });
                    return;
                }
                var results = data.map(function (r) {
                    return {
                        isError: false,
                        firstName: r.first_name,
                        lastName: r.last_name,
                        isMale: r.is_male,
                        firstNameCounts: r.first_name_writing_count_vec,
                        lastNameCounts: r.last_name_writing_count_vec,
                        tenUn: r.ten_un,
                        tiUn: r.ti_un,
                        jinUn: r.jin_un,
                        gaiUn: r.gai_un,
                        souUn: r.sou_un,
                        tenUnPoint: r.ten_un_point,
                        tiUnPoint: r.ti_un_point,
                        jinUnPoint: r.jin_un_point,
                        gaiUnPoint: r.gai_un_point,
                        souUnPoint: r.sou_un_point,
                        inyouPoint: r.in_myou_point,
                        totalPoint: r.total_point
                    };
                });
                this.setState({
                    results: results,
                    loaded: true
                });
            }.bind(this),
            error: function (xhr, status, err) {
                console.error(this.props.url, status, err.toString());
                this.setState({
                    isError: true,
                    loaded: true
                });
            }.bind(this)
        });
    },

    render: function () {
        var results = this.state.results.map(function (r) {
            return <CountResultItem key={r.firstNameCounts.toString()} lastName={r.lastName} firstName={r.firstName}
                                    isMale={r.isMale ? "男" : "女"}
                                    lastNameCounts={r.lastNameCounts} firstNameCounts={r.firstNameCounts}
                                    tenUn={r.tenUn} tiUn={r.tiUn} jinUn={r.jinUn} gaiUn={r.gaiUn} souUn={r.souUn}
                                    tenUnPoint={r.tenUnPoint * 10} tiUnPoint={r.tiUnPoint * 10}
                                    jinUnPoint={r.jinUnPoint * 10} gaiUnPoint={r.gaiUnPoint * 10}
                                    souUnPoint={r.souUnPoint * 10} inyouPoint={r.inyouPoint * 10}
                                    totalPoint={r.totalPoint * 10}/>
        });

        return (
            <div>
                <h2>よい画数を調べてみよう</h2>

                <p>名前をどの画数にしたら良い運勢になるか調べてみよう～<br/>地運, 人運, 外運, 総運が全て吉数になるパターンを調べることができます。</p>
                <CountForm onFormSubmit={this.handleFormSubmit} onChange={this.handleChange}
                           isError={this.state.isError}/>
                <Loader loaded={this.state.loaded}>
                    <table className="table table-bordered">
                        <thead>
                        <tr>
                            <th>名前画数 <br/>(地運画数)</th>
                            <th>文字数</th>
                            <th>各画数</th>
                            <th>天運画数</th>
                            <th>人運画数</th>
                            <th>外運画数</th>
                            <th>総運画数</th>
                            <th>天運</th>
                            <th>地運</th>
                            <th>人運</th>
                            <th>外運</th>
                            <th>総運</th>
                            <th>陰陽</th>
                            <th>トータル</th>
                        </tr>
                        </thead>
                        < tbody >
                        {results}
                        </tbody >
                    </table>
                </Loader>
            </div>
        );
    }
});

module.exports = CountResult;
