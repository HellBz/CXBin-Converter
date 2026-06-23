import { Button } from "@/components/ui/button";
import { useI18n } from "@/i18n";
import type { Language } from "@/i18n";

const FLAGS: Record<Language, string> = {
  en: "🇬🇧",
  de: "🇩🇪",
};

export function LanguageToggle() {
  const { language, setLanguage, t } = useI18n();

  const toggle = () => {
    setLanguage(language === "en" ? "de" : "en");
  };

  return (
    <Button
      variant="outline"
      size="icon"
      onClick={toggle}
      title={t("language.title", { lang: t(`language.${language}`) })}
    >
      {FLAGS[language]}
    </Button>
  );
}
