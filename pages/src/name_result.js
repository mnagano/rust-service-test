var React = require('react/addons');
var ReactART = require('react-art');
var Group = ReactART.Group;
var Shape = ReactART.Shape;
var Text = ReactART.Text;
var Surface = ReactART.Surface;
var Path = require('paths-js/path');

var NameResultImage = require('./name_result_image.js');
var TextInput = require('./name_input.js').textInput;
var RadioButton = require('./name_input.js').radioButton;

var NameForm = React.createClass({
    getInitialState: function () {
        return {
            firstName: '一郎',
            firstNameInputStatus: true,
            lastName: '長野',
            lastNameInputStatus: true,
            isMaleIndex: 0
        };
    },
    handleFirstNameChange: function (e, validationResult) {
        this.setState({
            firstName: e.target.value.trim(),
            firstNameInputStatus: validationResult,
        });
        this.props.onChange();
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
        var firstName = this.state.firstName.trim();
        var lastName = this.state.lastName.trim();
        var isMail = this.state.isMaleIndex == 0;
        this.props.onFormSubmit(firstName, lastName, isMail);
    },

    render: function () {
        var disabled = !this.state.lastNameInputStatus
            || !this.state.firstNameInputStatus;

        var errorText = this.props.isError ?
            "指定された名前には対応していません。おそらく名前に使えない字と考えられます。" : null;

        return (
            <form className="form-horizontal" onSubmit={this.handleSubmit}>
                <p className="text-danger bg-danger">{errorText}</p>
                <TextInput id="lastName" name="姓 : " placeholder="姓" reg={"^[^ -~｡-ﾟ]+$"} minLen={1} maxLen={5}
                           errorText="1から5文字以内の漢字で指定してください"
                           onChange={this.handleLastNameChange}>{this.state.lastName}</TextInput>
                <TextInput id="firstName" name="名 : " placeholder="名" reg={"^[^ -~｡-ﾟ]+$"} minLen={1} maxLen={5}
                           errorText="1から5文字以内の漢字で指定してください"
                           onChange={this.handleFirstNameChange}>{this.state.firstName}</TextInput>

                <RadioButton id="isMaleRadios" name="性別: " buttons={["男", "女"]} currentIndex={this.state.isMaleIndex}
                             onChange={this.handleIsMaleChange}/>

                <div className="form-group">
                    <div className="col-sm-offset-2 col-sm-10">
                        <button type="submit" className="btn btn-primary" disabled={disabled}>送る</button>
                    </div>
                </div>
            </form>
        );
    }
});

var NameResult = React.createClass({
    getInitialState: function () {
        return {
            firstName: '',
            lastName: '',
            isMale: true,
            firstNameCounts: [],
            lastNameCounts: [],
            tenUn: 0,
            tiUn: 0,
            jinUn: 0,
            gaiUn: 0,
            souUn: 0,
            tenUnPoint: 0,
            tiUnPoint: 0,
            jinUnPoint: 0,
            gaiUnPoint: 0,
            souUnPoint: 0,
            inyouPoint: 0,
            totalPoint: 0,
            isError: false,
            width: 0,
        };
    },

    handleChange: function () {
        this.setState({
            isError: false
        });
    },

    handleFormSubmit: function (firstName, lastName, isMale) {
        var url = '/api/names?'
            + 'first_name=' + firstName
            + '&last_name=' + lastName
            + '&is_male=' + isMale;

        //console.log(url);
        $.ajax({
            url: url,
            dataType: 'json',
            cache: false,
            success: function (data) {
                if (!data) {
                    this.setState({
                        isError: true
                    });
                    return;
                }
                this.setState({
                    isError: false,
                    firstName: data.first_name,
                    lastName: data.last_name,
                    isMale: data.is_male,
                    firstNameCounts: data.first_name_writing_count_vec,
                    lastNameCounts: data.last_name_writing_count_vec,
                    tenUn: data.ten_un,
                    tiUn: data.ti_un,
                    jinUn: data.jin_un,
                    gaiUn: data.gai_un,
                    souUn: data.sou_un,
                    tenUnPoint: data.ten_un_point * 10,
                    tiUnPoint: data.ti_un_point * 10,
                    jinUnPoint: data.jin_un_point * 10,
                    gaiUnPoint: data.gai_un_point * 10,
                    souUnPoint: data.sou_un_point * 10,
                    inyouPoint: data.in_myou_point * 10,
                    totalPoint: data.total_point * 10
                });
            }.bind(this),
            error: function (xhr, status, err) {
                console.error(this.props.url, status, err.toString());
                this.setState({
                    isError: true
                });
            }.bind(this)
        });
    },

    updateDimensions: function () {
        this.setState({
            width: React.findDOMNode(this).offsetWidth
        });
    },

    componentDidMount: function () {
        window.addEventListener("resize", this.updateDimensions);
    },

    render: function () {
        var styles = {
            color: "#ff7f7f",
            backgroundColor: "#bfff7f",
            fontSize: '16px',
            fontWeight: 'bold'
        }
        var className;
        var totalPoint = this.state.totalPoint;
        if (totalPoint > 80) {
            className = "success"
        } else if (totalPoint > 60) {
            className = "warning";
        } else if (totalPoint < 40) {
            className = "danger";
        }

        return (
            <div>
                <h2>名前で調べてみよう</h2>

                <p>付けたい名前を入れて、運勢を調べてみよう～</p>
                <NameForm onFormSubmit={this.handleFormSubmit} onChange={this.handleChange}
                          isError={this.state.isError}/>
                <NameResultImage nameResult={this.state} parentWidth={this.state.width}/>
                <table className="table table-striped table-bordered">
                    <thead>
                    <tr>
                        <th>姓</th>
                        <th style={styles}>名</th>
                        <th>性別</th>
                        <th>姓画数</th>
                        <th>名画数</th>
                        <th>天運画数</th>
                        <th>地運画数</th>
                        <th>人運画数</th>
                        <th>外運画数</th>
                        <th>総運画数</th>
                        <th>天運</th>
                        <th style={styles}>地運</th>
                        <th>人運</th>
                        <th>外運</th>
                        <th style={styles}>総運</th>
                        <th>陰陽</th>
                        <th style={styles}>トータル</th>
                    </tr>
                    </thead>
                    < tbody >
                    <tr className={className}>
                        <td>{this.state.lastName}</td>
                        <td style={styles}>{this.state.firstName}</td>
                        <td>{this.state.isMale ? "男" : "女"}</td>
                        <td>{this.state.lastNameCounts.toString()}</td>
                        <td>{this.state.firstNameCounts.toString()}</td>
                        <td>{this.state.tenUn}</td>
                        <td>{this.state.tiUn}</td>
                        <td>{this.state.jinUn}</td>
                        <td>{this.state.gaiUn}</td>
                        <td>{this.state.souUn}</td>
                        <td>{this.state.tenUnPoint}</td>
                        <td style={styles}>{this.state.tiUnPoint}</td>
                        <td>{this.state.jinUnPoint}</td>
                        <td>{this.state.gaiUnPoint}</td>
                        <td style={styles}>{this.state.souUnPoint}</td>
                        <td>{this.state.inyouPoint}</td>
                        <td style={styles}>{this.state.totalPoint.toString() + "点"}</td>
                    </tr>
                    </tbody >
                </table>
            </div>
        );
    }
});
module.exports = NameResult;
