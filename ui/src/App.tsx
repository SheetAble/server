import { Icon } from '@iconify/react';
import { useTranslation, } from 'react-i18next';
import { Button } from './components/Button';

function App() {
  const { t, i18n } = useTranslation();
  const changeLanguage = (lng: string) => {
    i18n.changeLanguage(lng);
  };

  return (
    <>
      {t('welcome')}
      <Icon icon="mdi:github" />
      <div className="flex flex-col gap-4">
        Current lanuage: {i18n.language}
        <Button onClick={() => changeLanguage('en-GB')}>English</Button>
        <Button onClick={() => changeLanguage('fr-FR')}>Français</Button>
        <Button onClick={() => changeLanguage('de-DE')}>German</Button>
      </div>
      <div>
        {i18n.language}
        <button type="button" onClick={() => changeLanguage('en-GB')}>English</button>
        <button type="button" onClick={() => changeLanguage('fr-FR')}>Français</button>
      </div>
    </>
  )
}

export default App
