var React = require('react/addons');
var ReactART = require('react-art');
var Path = require('paths-js/path');
var Surface = ReactART.Surface;
var Group = ReactART.Group;
var Shape = ReactART.Shape;
var Text = ReactART.Text;

var lineSize = 24 * 1.5;
var lineWidth = 10;
var textWidth = 30;
var marginWidth = 8;
var fontSize = 12;

var StyledText = React.createClass({
    getDefaultProps: function () {
        return {
            fill: '#404040',
            lineSize: lineSize,
            fontSize: 24,
            x: 0,
            lineIndex: 0
        };
    },
    getTextStyle: function () {
        return {
            x: this.props.x,
            y: this.props.lineSize * this.props.lineIndex + (this.props.lineSize - this.props.fontSize) / 2,
            fill: this.props.fill,
            font: 'bold ' + this.props.fontSize + 'px "Arial"'
        };
    },
    render: function () {
        return <Text {...this.getTextStyle()}>{this.props.children}</Text>;
    }
});

var Line = React.createClass({
    propTypes: {
        x1: React.PropTypes.number,
        lineIndex1: React.PropTypes.number.isRequired,
        x2: React.PropTypes.number,
        lineIndex2: React.PropTypes.number
    },
    render: function () {
        var x1 = this.props.x1 || 0;
        var x2 = this.props.x2 || x1;
        var lineIndex2 = this.props.lineIndex2 || this.props.lineIndex1;
        var path = Path().moveto(this.props.x1, (this.props.lineIndex1 + 0.5) * lineSize).lineto(x2, (lineIndex2 + 0.5) * lineSize);
        return <Shape d={path.print()} strokeWidth={1} stroke="#FF7F7F"/>;
    }
});

var Name = React.createClass({
    propTypes: {
        lastName: React.PropTypes.string.isRequired,
        firstName: React.PropTypes.string.isRequired
    },
    render: function () {
        var texts = [];
        if (!this.props.lastName || !this.props.firstName) return null;
        for (var i = 0; i < this.props.lastName.length; i++) {
            texts.push(<StyledText lineIndex={i}>{this.props.lastName.charAt(i)}</StyledText>);
        }
        var offset = this.props.lastName.length + 1;
        for (var i = 0; i < this.props.firstName.length; i++) {
            texts.push(<StyledText lineIndex={offset + i}>{this.props.firstName.charAt(i)}</StyledText>);
        }
        return <Group>{texts}</Group>;
    }
});

var WritingCounts = React.createClass({
    propTypes: {
        lastNameCounts: React.PropTypes.array.isRequired,
        firstNameCounts: React.PropTypes.array.isRequired
    },
    render: function () {
        var texts = [];
        if (this.props.lastNameCounts && this.props.firstNameCounts) {
            for (var i = 0; i < this.props.lastNameCounts.length; i++) {
                texts.push(<StyledText fontSize='16' lineIndex={i}>{this.props.lastNameCounts[i] + "画"}</StyledText>);
            }
            var offset = this.props.lastNameCounts.length + 1;
            for (var i = 0; i < this.props.firstNameCounts.length; i++) {
                texts.push(<StyledText fontSize='16'
                                       lineIndex={offset + i}>{this.props.firstNameCounts[i] + "画"}</StyledText>);
            }
        }
        return <Group>{texts}</Group>;
    }
});

var LuckyResult = React.createClass({
    propTypes: {
        point: React.PropTypes.number.isRequired,
        x: React.PropTypes.number,
        lineIndex: React.PropTypes.number
    },
    render: function () {
        var pointText;
        var fill;
        switch (this.props.point) {
            case -20 :
                pointText = "大凶";
                fill = "#000"
                break;
            case -10 :
                pointText = "凶";
                fill = "#7f7fff"
                break;
            case 0 :
                pointText = "小吉";
                fill = "#bf7fff"
                break;
            case 10 :
                pointText = "吉";
                fill = "#ff7fff"
                break;
            case 20 :
                pointText = "中吉";
                fill = "#ff7fbf"
                break;
            case 30 :
                pointText = "大吉";
                fill = "#ff7f7f"
                break;
            default:
                break;
        }
        return <StyledText fontSize='16' fill={fill} x={this.props.x}
                           lineIndex={this.props.lineIndex}>{pointText}</StyledText>
    }
});

var Un = React.createClass({
    propTypes: {
        writingCount: React.PropTypes.number.isRequired,
        point: React.PropTypes.number.isRequired,
        lineIndex: React.PropTypes.number.isRequired,
    },

    render: function () {
        var texts = [];
        var results = [];
        //運 + 画数
        var lineIndex = this.props.lineIndex;
        texts.push(<StyledText fontSize={fontSize} lineIndex={lineIndex-0.6}>{this.props.children}</StyledText>);
        texts.push(<StyledText fontSize={fontSize}
                               lineIndex={lineIndex-0.1}>{this.props.writingCount.toString() + "画"}</StyledText>);
        texts.push(<LuckyResult lineIndex={lineIndex+0.5} point={this.props.point}/>);
        return (<Group>
            {texts}
        </Group>)
    }
});

var TenUn = React.createClass({
    propTypes: {
        lastName: React.PropTypes.string.isRequired,
        writingCount: React.PropTypes.number.isRequired,
        point: React.PropTypes.number.isRequired,
    },

    render: function () {
        var lines = [];
        //線
        for (var i = 0; i < this.props.lastName.length; i++) {
            lines.push(<Line x1={0} x2={lineWidth} lineIndex1={i}/>);
        }
        lines.push(<Line x1={lineWidth} lineIndex1={0} lineIndex2={this.props.lastName.length-1}/>);
        var offsetY = (this.props.lastName.length - 1) / 2;
        return (<Group>
            {lines}
            <Group x={lineWidth + marginWidth}>
                <Un writingCount={this.props.writingCount} lineIndex={offsetY} point={this.props.point}>天運</Un>
            </Group>
        </Group>)
    }
});

var TiUn = React.createClass({
    propTypes: {
        lastName: React.PropTypes.string.isRequired,
        firstName: React.PropTypes.string.isRequired,
        writingCount: React.PropTypes.number.isRequired,
        point: React.PropTypes.number.isRequired,
    },

    render: function () {
        var lines = [];
        //線
        var offsetY = this.props.lastName.length + 1;
        var name = this.props.firstName;
        for (var i = 0; i < name.length; i++) {
            lines.push(<Line x1={0} x2={lineWidth} lineIndex1={offsetY+i}/>);
        }
        lines.push(<Line x1={lineWidth} lineIndex1={offsetY} lineIndex2={offsetY+name.length-1}/>);
        offsetY += (name.length - 1) / 2;
        return (<Group>
            {lines}
            <Group x={lineWidth + marginWidth}>
                <Un writingCount={this.props.writingCount} lineIndex={offsetY} point={this.props.point}>地運</Un>
            </Group>
        </Group>)
    }
});

var SouUn = React.createClass({
    propTypes: {
        lastName: React.PropTypes.string.isRequired,
        firstName: React.PropTypes.string.isRequired,
        writingCount: React.PropTypes.number.isRequired,
        point: React.PropTypes.number.isRequired,
    },

    render: function () {
        var lines = [];
        //線
        var offsetY1 = (this.props.lastName.length - 1) / 2;
        lines.push(<Line x1={0} x2={lineWidth} lineIndex1={offsetY1}/>);
        var offsetY2 = this.props.lastName.length + 1 + (this.props.firstName.length - 1) / 2;
        lines.push(<Line x1={0} x2={lineWidth} lineIndex1={offsetY2}/>);
        lines.push(<Line x1={lineWidth} lineIndex1={offsetY1} lineIndex2={offsetY2}/>);
        offsetY = (offsetY1 + offsetY2) / 2;
        return (<Group>
            {lines}
            <Group x={lineWidth + marginWidth}>
                <Un writingCount={this.props.writingCount} lineIndex={offsetY} point={this.props.point}>総運</Un>
            </Group>
        </Group>)
    }
});

var JinUn = React.createClass({
    propTypes: {
        lastName: React.PropTypes.string.isRequired,
        firstName: React.PropTypes.string.isRequired,
        writingCount: React.PropTypes.number.isRequired,
        point: React.PropTypes.number.isRequired,
    },

    render: function () {
        var lines = [];
        //線
        var offsetY1 = this.props.lastName.length - 1;
        lines.push(<Line x1={0} x2={lineWidth} lineIndex1={offsetY1}/>);
        var offsetY2 = this.props.lastName.length + 1;
        lines.push(<Line x1={0} x2={lineWidth} lineIndex1={offsetY2}/>);
        lines.push(<Line x1={0} lineIndex1={offsetY1} lineIndex2={offsetY2}/>);
        //画数
        var offsetY = (offsetY1 + offsetY2) / 2;
        return (<Group>
            <Group x={textWidth+marginWidth}>{lines}</Group>
            <Un writingCount={this.props.writingCount} lineIndex={offsetY} point={this.props.point}>人運</Un>
        </Group>)
    }
});

var GaiUn = React.createClass({
    propTypes: {
        lastName: React.PropTypes.string.isRequired,
        firstName: React.PropTypes.string.isRequired,
        writingCount: React.PropTypes.number.isRequired,
        point: React.PropTypes.number.isRequired,
    },

    render: function () {
        var lines = [];
        //線
        var name = this.props.lastName;
        //姓
        // 1文字のときは最初の文字から
        var offsetY1;
        offsetY1 = 0;
        var width = lineWidth + marginWidth * 2 + textWidth;
        if (name.length == 1) {
            lines.push(<Line x1={0} x2={width} lineIndex1={0}/>);
        } else {
            for (var i = 0; i < name.length - 1 && i < 2; i++) {
                lines.push(<Line x1={0} x2={width} lineIndex1={i}/>);
            }
        }

        //名
        // 1文字のときは最初の文字から
        var offsetY = name.length + 1;
        var name = this.props.firstName;
        var offsetY2;
        if (name.length == 1) {
            lines.push(<Line x1={0} x2={width} lineIndex1={offsetY}/>);
            offsetY2 = offsetY;
        } else {
            offsetY2 = offsetY + name.length - 1;
            for (var i = 1; i < name.length; i++) {
                if (i < (name.length - 2)) continue;
                lines.push(<Line x1={0} x2={width} lineIndex1={offsetY+i}/>);
            }
        }
        lines.push(<Line x1={0} lineIndex1={offsetY1} lineIndex2={offsetY2}/>);
        var offsetY = (offsetY1 + offsetY2) / 2;
        return (<Group>
            <Group x={textWidth+marginWidth}>{lines}</Group>
            <Un writingCount={this.props.writingCount} lineIndex={offsetY} point={this.props.point}>外運</Un>
        </Group>)
    }
});

var PointImage = React.createClass({
    render: function () {
        var point = this.props.point;
        var fill;
        var pointText;
        if (point >= 100) {
            pointText = point.toString() + "点!!";
            fill = "#ff7f7f";
        } else if (point > 50) {
            pointText = point.toString() + "点!!";
            fill = "#ff7fbf";
        } else if (point > 30) {
            pointText = point.toString() + "点!!";
            fill = "#bf7fff";
        } else if (point > 0) {
            pointText = point.toString() + "点!!";
            fill = "#7f7fff";
        } else {
            pointText = point.toString() + "点";
            fill = "#404040";
        }

        var textStyle = {
            x: 0,
            y: 0,
            fill: fill,
            font: 'bold 60px "Arial"'
        };

        var texts = []
        texts.push(<Text {...textStyle}>{pointText}</Text>);
        return <Group>{texts}</Group>;
    }
});


var NameResultImage = React.createClass({
    render: function () {
        var nameResult = this.props.nameResult;
        var texts = [];
        var nameY = 0;

        if (this.props.nameResult.lastNameCounts.length == 0) return null;

        var offsetX = 0;
        texts.push(
            <Group x={offsetX} y={nameY}>
                <GaiUn firstName={nameResult.firstName} lastName={nameResult.lastName} writingCount={nameResult.gaiUn}
                       point={nameResult.gaiUnPoint}/>
            </Group>
        );
        offsetX += marginWidth * 2 + textWidth * 1;

        texts.push(
            <Group x={offsetX} y={nameY}>
                <JinUn firstName={nameResult.firstName} lastName={nameResult.lastName} writingCount={nameResult.jinUn}
                       point={nameResult.jinUnPoint}/>
            </Group>
        );
        offsetX += lineWidth + marginWidth * 2 + textWidth;

        var nameX = offsetX;
        texts.push(
            <Group x={offsetX} y={nameY}>
                <Name firstName={nameResult.firstName} lastName={nameResult.lastName}/>
            </Group>
        )
        offsetX += textWidth + marginWidth;

        texts.push(
            <Group x={offsetX} y={nameY}>
                <WritingCounts firstNameCounts={nameResult.firstNameCounts}
                               lastNameCounts={nameResult.lastNameCounts}/>
            </Group>
        );
        offsetX += textWidth + marginWidth;

        texts.push(
            <Group x={offsetX} y={nameY}>
                <TenUn lastName={nameResult.lastName} writingCount={nameResult.tenUn} point={nameResult.tenUnPoint}/>
            </Group>
        );

        texts.push(
            <Group x={offsetX} y={nameY}>
                <TiUn firstName={nameResult.firstName} lastName={nameResult.lastName} writingCount={nameResult.tiUn}
                      point={nameResult.tiUnPoint}/>
            </Group>
        );
        offsetX += lineWidth + marginWidth * 2 + textWidth;

        texts.push(
            <Group x={offsetX} y={nameY}>
                <SouUn firstName={nameResult.firstName} lastName={nameResult.lastName} writingCount={nameResult.souUn}
                       point={nameResult.souUnPoint}/>
            </Group>
        );
        var windowWidth = this.props.parentWidth == 0 ? window.innerWidth : this.props.parentWidth;
        var imageWidth = offsetX + lineWidth + marginWidth * 1 + textWidth
        var imageHeight = 300;
        var scale;
        // スクロールバー分引く
        var scaleX = (windowWidth - 50) / imageWidth;
        scale = scaleX;
        if (scale > 2) scale = 2;
        var surfaceWidth = scale * imageWidth;
        var surfaceHeight = scale * imageHeight;
        var surfaceLeft = (windowWidth - surfaceWidth) / 2;
        return (
            <Surface width={windowWidth} height={surfaceHeight} left={0} top={0}>
                <Group x={surfaceLeft+(nameX-30)*scale} y={20} scaleX={scale} scaleY={scale}>
                    <PointImage point={nameResult.totalPoint}/>
                </Group>
                <Group x={surfaceLeft} y={100*scale} scaleX={scale} scaleY={scale}>
                    {texts}
                </Group>
            </Surface>
        );
    }
});

module.exports = NameResultImage;
