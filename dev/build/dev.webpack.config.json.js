const webpack = require('webpack');
const wasm_pack_plugin = require('@wasm-tool/wasm-pack-plugin');
const html_webpack_plugin = require('html-webpack-plugin');
const path = require('path');

module.exports = (env, args) => {
    const is_production = (args.mode === 'production');

    return {
        mode: args.mode,
        entry: './../../client/index.js',
        output: {
            path: path.resolve(__dirname, '../../public'),
            filename: is_production ?
                '[name].[contenthash].js' :
                '[name].[name].js'
        },
        resolve: {
            modules: [path.resolve(__dirname, '../../dev/build/node_modules'), 'node_modules']
        },
        plugins: [
            new html_webpack_plugin({
                template: '../../client/index.html'
            }),
            new wasm_pack_plugin({
                crateDirectory: path.resolve(__dirname, '../..'),
                outDir: path.resolve(__dirname, '../../public/pkg')
            }),
            new webpack.ProvidePlugin({
                TextDecoder: ['text-encoding', 'TextDecoder'],
                TextEncoder: ['text-encoding', 'TextEncoder']
            })
        ],
        module: {
            rules: [
                {
                    test: /\.css$/i,
                    use: ["style-loader", "css-loader"]
                }
            ]
        },
        experiments: {
            asyncWebAssembly: true
        }
    }
}