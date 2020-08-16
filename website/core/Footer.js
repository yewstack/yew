/**
 * Copyright (c) 2017-present, Facebook, Inc.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

const React = require('react');

class Footer extends React.Component {
  docUrl(doc, language) {
    const baseUrl = this.props.config.baseUrl;
    const docsUrl = this.props.config.docsUrl;
    const docsPart = `${docsUrl ? `${docsUrl}/` : ''}`;
    const langPart = `${language ? `${language}/` : ''}`;
    return `${baseUrl}${docsPart}${langPart}${doc}`;
  }

  pageUrl(doc, language) {
    const baseUrl = this.props.config.baseUrl;
    return baseUrl + (language ? `${language}/` : '') + doc;
  }

  render() {
    return (
      <footer className="nav-footer" id="footer">
        <section className="sitemap">
          <a href={this.props.config.baseUrl} className="nav-home">
            {this.props.config.footerIcon && (
              <img
                src={this.props.config.baseUrl + this.props.config.footerIcon}
                alt={this.props.config.title}
                width="66"
                height="58"
              />
            )}
          </a>

          <div>
            <h5>Support</h5>
            <a
              href="https://issuehunt.io/r/yewstack/yew"
              target="_blank"
              rel="noreferrer noopener">
              Fund Issues
            </a>
            <a
              href="https://opencollective.com/yew"
              target="_blank"
              rel="noreferrer noopener">
              Sponsor Project
            </a>
          </div>

          <div>
            <h5>Participate</h5>
            <a
              href={this.props.config.repoUrl}
              target="_blank"
              rel="noreferrer noopener">
              Github
            </a>
            <a
              href="https://discord.gg/VQck8X4"
              target="_blank"
              rel="noreferrer noopener">
              Discord
            </a>
            <a
              href={`https://twitter.com/${this.props.config.twitterUsername}`}
              target="_blank"
              rel="noreferrer noopener">
              Twitter
            </a>
          </div>

          <div>
            <h5>More</h5>
            <a
              href="https://github.com/jetli/awesome-yew"
              target="_blank"
              rel="noreferrer noopener">
              Yew Awesome
            </a>
          </div>
        </section>
      </footer>
    );
  }
}

module.exports = Footer;
