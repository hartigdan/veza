const path = require('path')
const HtmlWebpackPlugin = require('html-webpack-plugin')
const { VueLoaderPlugin } = require('vue-loader')

module.exports = {
    mode: 'development',
    entry: './main.js',
    output: {
        filename: 'bundle.js',
        path: path.resolve(__dirname, 'dist')
    },
    module: {
        rules: [{
            test: /\.vue$/,
            use: [
                'vue-loader'
            ]
        }, {
            test: /\.css$/,
            use: [
                'vue-style-loader',
                'css-loader'
            ]
        }, {
            test: /\.js$/,
            exclude: /node_modules/,
            use: [
                'babel-loader'
            ]
        }, {
            test: /\.(ijmap|svg|ttf|woff|woff2|eot)$/i,
            loader: 'file-loader'
        }]
    },
    plugins: [
        new VueLoaderPlugin(),
        new HtmlWebpackPlugin()
    ]
}
