import type { ReactNode } from "react";
import { useI18n } from "@/i18n";
import type { Language } from "@/i18n";
import { UkFlag, DeFlag, FrFlag, EsFlag, CnFlag, JpFlag } from "@/components/flags";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";

const LANGUAGE_FLAGS: Record<Language, ReactNode> = {
  en: <UkFlag className="h-5 w-5" />,
  de: <DeFlag className="h-5 w-5" />,
  fr: <FrFlag className="h-5 w-5" />,
  es: <EsFlag className="h-5 w-5" />,
  zh: <CnFlag className="h-5 w-5" />,
  ja: <JpFlag className="h-5 w-5" />,
};

const LANGUAGE_ORDER: Language[] = ["en", "de", "fr", "es", "zh", "ja"];

export function LanguageToggle() {
  const { language, setLanguage, t } = useI18n();
  const currentLabel = t(`language.${language}`);

  return (
    <Select value={language} onValueChange={(value) => setLanguage(value as Language)}>
      <SelectTrigger className="w-40 gap-2">
        <SelectValue placeholder={currentLabel} />
      </SelectTrigger>
      <SelectContent>
        {LANGUAGE_ORDER.map((value) => (
          <SelectItem key={value} value={value}>
            <div className="flex items-center gap-2">
              {LANGUAGE_FLAGS[value]}
              <span>{t(`language.${value}`)}</span>
            </div>
          </SelectItem>
        ))}
      </SelectContent>
    </Select>
  );
}
