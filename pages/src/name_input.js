var React = require('react/addons');

var TextInput = React.createClass({
    getDefaultProps: function () {
        return {
            reg: "^[0-9]+",
        }
    },
    propTypes: {
        id: React.PropTypes.string.isRequired,
        name: React.PropTypes.string.isRequired,
        placeHolder: React.PropTypes.string,
        onChange: React.PropTypes.func,
        max: React.PropTypes.number,
        min: React.PropTypes.number,
        maxLen: React.PropTypes.number,
        minLen: React.PropTypes.number,
        reg: React.PropTypes.string,
        errorText: React.PropTypes.string,
    },
    getInitialState: function () {
        return {
            status: true,
        }
    },

    validate: function(val){
        var re = new RegExp(this.props.reg);
        if (!re.test(val)) return false;
        if (this.props.max != null && Number(val) > this.props.max) return false;
        if (this.props.min != null && Number(val) < this.props.min) return false;
        if (this.props.minLen != null && val.length < this.props.minLen) return false;
        if (this.props.maxLen != null && val.length > this.props.maxLen) return false;
        return true;
    },

    handleChange: function (e) {
        var validationResult = this.validate(e.target.value);
        this.setState({
            status: validationResult
        });
        this.props.onChange(e, validationResult);
    },

    render: function () {
        var errorText;
        var formClass;
        var iconClass;
        if (this.props.children !== "") {
            if (this.validate(this.props.children)) {
                formClass = "form-group has-success has-feedback";
                iconClass = "glyphicon glyphicon-ok form-control-feedback";
            } else {
                formClass = "form-group has-error has-feedback";
                iconClass = "glyphicon glyphicon-remove form-control-feedback";
                errorText = this.props.errorText;
                if (errorText == null) {
                    errorText = "";
                    if (this.props.maxLen == null) {
                        if (this.props.min != null) {
                            errorText = errorText + this.props.min.toString() + "以上"
                        }
                        if (this.props.max != null) {
                            errorText = errorText + this.props.max.toString() + "以下"
                        }
                        if (errorText) errorText = errorText + "の";
                        errorText = errorText + "数字を入力してください"
                    } else {
                        if (this.props.minLen != null) {
                            errorText = errorText + this.props.minLen.toString() + "以上"
                        }
                        if (this.props.maxLen != null) {
                            errorText = errorText + this.props.maxLen.toString() + "以下"
                        }
                        errorText = errorText + "の文字列で入力してください"
                    }
                }
            }
        } else {
            formClass = "form-group";
        }
        return (
            <div className={formClass}>
                <label htmlFor={this.props.id} className="col-sm-2 control-label">{this.props.name}</label>

                <div className="col-sm-10">
                    <input type="text" className="form-control" id={this.props.id} placeholder={this.props.placeHolder}
                           value={this.props.children} onChange={this.handleChange}/>
                    <span className={iconClass} aria-hidden="true"></span>

                    <p className="bg-danger">{errorText}</p>
                </div>
            </div>
        );
    }
});

var RadioButtonItem = React.createClass({
    propTypes: {
        id: React.PropTypes.string.isRequired,
        name: React.PropTypes.string.isRequired,
        index: React.PropTypes.number.isRequired,
        currentIndex: React.PropTypes.number.isRequired,
        onChange: React.PropTypes.func,
    },

    handleChange: function (e) {
        this.props.onChange(e, this.props.index);
    },

    render: function () {
        var value;
        if (this.props.currentIndex === this.props.index) {
            value = true;
        }
        return (
            <label className="radio-inline">
                <input type="radio" name={this.props.name} id={this.props.id}
                       onChange={this.handleChange} value={value}
                       defaultChecked={value}>{this.props.children}</input>
            </label>
        );
    }
});

var RadioButton = React.createClass({
    propTypes: {
        id: React.PropTypes.string.isRequired,
        currentIndex: React.PropTypes.number.isRequired,
        name: React.PropTypes.string.isRequired,
        onChange: React.PropTypes.func,
    },

    handleChange: function (e, index) {
        this.props.onChange(e, index);
    },

    render: function () {
        var buttons = this.props.buttons.map(function (val, index) {
            return <RadioButtonItem key={this.props.id + index} id={this.props.id + index} name={this.props.id}
                                    index={index}
                                    currentIndex={this.props.currentIndex}
                                    onChange={this.handleChange}>{val}</RadioButtonItem>
        }.bind(this));
        return (
            <div className="form-group">
                <label htmlFor={this.props.id} className="col-sm-2 control-label">{this.props.name}</label>
                {buttons}
            </div>
        );
    }
});

module.exports = {
    textInput: TextInput,
    radioButton: RadioButton
};
