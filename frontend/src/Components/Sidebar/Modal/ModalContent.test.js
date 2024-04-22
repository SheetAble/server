import React from "react";
import { Provider } from "react-redux";
import { render } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import configureMockStore from "redux-mock-store";
import { shallow } from "enzyme";
import ModalContent from "./ModalContent";

const mockStore = configureMockStore();
const store = mockStore({
  user: {},
  UI: {},
});
const uploadMock = jest.fn();

describe("ModalContent component", () => {
  it("Renders without errors", () => {
    render(
      <Provider store={store}>
        <ModalContent />
      </Provider>
    );
  });
});

test("Submits the form when Upload button is clicked", () => {
  const user = userEvent.setup();
  const propsMock = jest.fn();
  const { getByLabelText, getByText } = render(
    <Provider store={store}>
      <ModalContent props={propsMock} />
    </Provider>
  );
  const sheetNameInput = getByLabelText("Sheet Name");
  const composerInput = getByLabelText("Composer");
  const uploadButton = getByText("Upload");

  user.type(sheetNameInput, "test sheet name");
  user.type(composerInput, "test composer name");
  user.click(uploadButton);

  expect(propsMock.uploadSheet);
  expect(propsMock.resetData);
});
