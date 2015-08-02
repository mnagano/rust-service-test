var React = require('react/addons');
var Loader = require('react-loader');
var TextInput = require('./name_input.js').textInput;
var RadioButton = require('./name_input.js').radioButton;

var NameListForm = React.createClass({
    getInitialState: function () {
        return {
            writingCount: 5,
            writingCountInputStatus: true,
            lastName: '長野',
            lastNameInputStatus: true,
            isMaleIndex: 0,
            offset: 0,
            offsetInputStatus: true,
            limit: 200,
            limitInputStatus: true,
        };
    },
    handleLimitChange: function (e, validationResult) {
        this.setState({
            limit: e.target.value,
            limitInputStatus: validationResult,
        });
    },
    handleOffsetChange: function (e, validationResult) {
        this.setState({
            offset: e.target.value,
            offsetInputStatus: validationResult,
        });
    },
    handleWritingCountChange: function (e, validationResult) {
        this.setState({
            writingCount: e.target.value,
            offsetInputStatus: validationResult,
        });
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
    handleSubmit: function (e) {
        e.preventDefault();
        var writingCount = this.state.writingCount;
        var lastName = this.state.lastName.trim();
        var isMail = this.state.isMaleIndex == 0;
        var offset = this.state.offset;
        var limit = this.state.limit;
        this.props.onFormSubmit(lastName, isMail, writingCount, offset, limit);
    },
    render: function () {
        var disabled = !this.state.lastNameInputStatus
            || !this.state.limitInputStatus
            || !this.state.offsetInputStatus
            || !this.state.writingCountInputStatus;

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
                <TextInput id="writingCount" name="名前の画数 : " placeholder="名前の画数" min={1} max={81}
                           onChange={this.handleWritingCountChange}>{this.state.writingCount}</TextInput>
                <TextInput id="offset" name="offset : " placeholder="オフセット(0スタート)" min={0} max={99999}
                           onChange={this.handleOffsetChange}>{this.state.offset}</TextInput>
                <TextInput id="limit" name="limit : " placeholder="取得個数" min={1} max={2000}
                           onChange={this.handleLimitChange}>{this.state.limit}</TextInput>

                <div className="form-group">
                    <div className="col-sm-offset-2 col-sm-10">
                        <button type="submit" className="btn btn-primary" disabled={disabled}>送る</button>
                    </div>
                </div>
            </form>
        );
    }
});

var NameListResultItem = React.createClass({
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
                <td style={styles}>{this.props.firstName}</td>
                <td>{this.props.firstNameCounts.toString()}</td>
                <td>{this.props.tenUn}</td>
                <td>{this.props.tiUn}</td>
                <td>{this.props.jinUn}</td>
                <td>{this.props.gaiUn}</td>
                <td>{this.props.souUn}</td>
                <td>{this.props.tenUnPoint}</td>
                <td style={styles}>{this.props.tiUnPoint}</td>
                <td>{this.props.jinUnPoint}</td>
                <td>{this.props.gaiUnPoint}</td>
                <td style={styles}>{this.props.souUnPoint}</td>
                <td>{this.props.inyouPoint}</td>
                <td style={styles}>{this.props.totalPoint.toString() + "点"}</td>
            </tr>
        );
    }
});

var NameListResult = React.createClass({
    getInitialState: function () {
        return {
            results: [],
            loaded: true
        };
    },

    handleFormSubmit: function (lastName, isMale, writingCount, offset, limit) {
        this.setState({
            loaded: false
        });
        var url = '/api/names?'
            + 'last_name=' + lastName
            + '&writing_count=' + writingCount
            + '&is_male=' + isMale
            + '&offset=' + offset
            + '&limit=' + limit;

        //console.log(url);
        $.ajax({
            url: url,
            dataType: 'json',
            cache: false,
            success: function (data) {
                if (!data) {
                    this.setState({
                        results: [],
                        isError: true,
                        loaded: true
                    });
                    return;
                }
                if (data.length == 0) {
                    this.setState({
                        results: [],
                        isNotFound: true,
                        loaded: true
                    });
                    return;
                }
                var results = data.map(function (r) {
                    return {
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
                        totalPoint: r.total_point,
                        isError: false,
                        isNotFound: false
                    };
                });
                this.setState({
                    results: results,
                    isNotFound: false,
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

    handleChange: function () {
        this.setState({
            isError: false,
            isNotFound: false
        });
    },

    render: function () {
        var results = this.state.results.map(function (r) {
            return <NameListResultItem key={r.lastName+r.firstName} lastName={r.lastName} firstName={r.firstName}
                                       isMale={r.isMale ? "男" : "女"}
                                       lastNameCounts={r.lastNameCounts.toString()}
                                       firstNameCounts={r.firstNameCounts.toString()} tenUn={r.tenUn} tiUn={r.tiUn}
                                       jinUn={r.jinUn} gaiUn={r.gaiUn} souUn={r.souUn} tenUnPoint={r.tenUnPoint * 10}
                                       tiUnPoint={r.tiUnPoint * 10} jinUnPoint={r.jinUnPoint * 10}
                                       gaiUnPoint={r.gaiUnPoint * 10} souUnPoint={r.souUnPoint * 10}
                                       inyouPoint={r.inyouPoint * 10} totalPoint={r.totalPoint * 10}/>
        });

        var errorText = this.state.isNotFound ?
            "地運, 人運, 外運, 総運が吉数になる名前は見つかりませんでした" : null;
        return (
            <div>
                <h2>画数で検索してみよう</h2>

                <p>画数を指定すると良い運勢の名前の候補が調べられるよ～<br/>地運, 人運, 外運, 総運が全て吉数になるパターンを調べることができます。</p>
                <NameListForm onFormSubmit={this.handleFormSubmit} onChange={this.handleChange}
                              isError={this.state.isError}/>

                <p className="text-danger bg-danger">{errorText}</p>
                <Loader loaded={this.state.loaded}>
                <table className="table table-striped table-bordered">
                    <thead>
                    <tr>
                        <th>名前</th>
                        <th>各文字の画数</th>
                        <th>天運画数</th>
                        <th>地運画数</th>
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

module.exports = NameListResult;
