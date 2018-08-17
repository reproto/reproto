const HtmlWebpackPlugin = require("html-webpack-plugin");
const CopyWebpackPlugin = require("copy-webpack-plugin");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const OptimizeCSSAssetsPlugin = require("optimize-css-assets-webpack-plugin");
const UglifyJsPlugin = require("uglifyjs-webpack-plugin");

module.exports = {
  entry: [
    "./src/index.tsx",
    "./src/main.scss",
  ],

  output: {
    filename: "bundle.js",
    path: __dirname + "/dist"
  },

  resolve: {
    extensions: [".ts", ".tsx", ".js", ".wasm"],
    modules: ["node_modules", "local_modules"],
  },

  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: [
          {
            loader: "babel-loader",
            options: { presets: ['@babel/preset-env'] }
          },
          "awesome-typescript-loader",
        ],
      },
      {
        test: /\.js?$/,
        loader: "babel-loader",
        exclude: /(node_modules|bower_components)/,
        options: {
          presets: ['@babel/preset-env'],
          plugins: ['@babel/plugin-syntax-dynamic-import']
        }
      },
      {
        test: /\.scss$/,
        use: [
          MiniCssExtractPlugin.loader,
          "css-loader",
          "sass-loader"
        ]
      },
      { test: /\.(jpe?g|gif|png)$/, loader: "file-loader" },
      { test: /.(ttf|otf|eot|svg|woff(2)?)(\?[a-z0-9]+)?$/,
        use: [{
          loader: 'file-loader',
          options: {
            name: '[name].[ext]',
            outputPath: 'fonts/',
            publicPath: '/fonts/'
          }
        }]
      }
    ]
  },

  plugins: [
    new HtmlWebpackPlugin({
      template: 'index.html'
    }),
    new CopyWebpackPlugin([
      'src/static/favicon.ico',
    ]),
    new MiniCssExtractPlugin({
      filename: "[name].css",
      chunkFilename: "[id].css"
    })
  ],

  optimization: {
    minimizer: [
      new UglifyJsPlugin({
        cache: true,
        parallel: true,
        sourceMap: true
      }),
      new OptimizeCSSAssetsPlugin({})
    ]
  },

  mode: 'development',
};
