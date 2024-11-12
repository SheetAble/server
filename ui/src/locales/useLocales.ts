import i18n from "i18next";
import { initReactI18next } from "react-i18next";
import gbTranslation from './en-GB.json'
import frTranslation from './fr-FR.json'

const resources = {
  gb: {
    translation: gbTranslation
  },
  fr: {
    translation: frTranslation
  }
};

i18n
  .use(initReactI18next)
  .init({
    resources,
    lng: "en-GB",
    interpolation: {
      escapeValue: false
    },
  });

  export default i18n;