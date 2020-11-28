const React = require("react");

const CompLibrary = require("../../core/CompLibrary");

const Container = CompLibrary.Container;

function NotFound() {
  return (
    <div className="docMainWrapper wrapper">
      <Container className="mainContainer documentContainer postContainer">
        <div className="post">
          <header className="postHeader">
            <h1>Not Found</h1>
          </header>
          The page you are looking for does not exist.
        </div>
      </Container>
    </div>
  );
}

module.exports = NotFound;
