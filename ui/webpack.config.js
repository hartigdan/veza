const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const { VueLoaderPlugin } = require('vue-loader');
const distDir = path.resolve(__dirname, 'dist');

module.exports = {
    mode: 'development',
    entry: './main.js',
    output: {
        filename: 'bundle.js',
        path: distDir,
    },
    module: {
        rules: [{
            test: /\.vue$/i,
            use: [
                'vue-loader',
            ]
        }, {
            test: /\.css$/i,
            use: [
                'vue-style-loader',
                'css-loader',
            ]
        }, {
            test: /\.js$/i,
            exclude: /node_modules/,
            use: {
                loader: 'babel-loader',
                options: {
                    presets: ['@babel/preset-env'],
                }
            }
        }, {
            test: /\.(woff|woff2)$/i,
            type: 'asset/resource',
        }]
    },
    plugins: [
        new VueLoaderPlugin(),
        new HtmlWebpackPlugin({
            title: 'Veza',
        }),
    ],
    devServer: {
        static: {
            directory: distDir
        },
        compress: true,
        port: 8080,
    },
};
