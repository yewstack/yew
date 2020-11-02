const React = require("react");

const CompLibrary = require("../../core/CompLibrary");

const Container = CompLibrary.Container;

function Home(props) {
  const { config, language } = props;
  const langPath = language ? language + "/" : "";
  const href = `${config.baseUrl}${config.docsUrl}/${langPath}`;

  return (
    <div className="docMainWrapper wrapper">
      {/* not ideal, but it's the best we can do with Docusaurus v1 */}
      <script
        dangerouslySetInnerHTML={{
          __html: `window.location.href = "${href}";`,
        }}
      ></script>
      <Container className="mainContainer documentContainer postContainer">
        <div className="post">
          <header className="postHeader">
            <h1>Redirecting</h1>
          </header>
          <div>
            {/* please note the trailing space */}
            {"If you are not redirected automatically, follow this "}
            <a href={href}>link</a>.
          </div>
        </div>
      </Container>
    </div>
  );
}

module.exports = Home;
