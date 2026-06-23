import { createContext, useContext, useState, type ReactNode } from "react";
import type { Language } from "./translations";
import { translations } from "./translations";

const STORAGE_KEY = "cxbin-language";

interface I18nContextType {
  language: Language;
  setLanguage: (lang: Language) => void;
  t: (key: string, replacements?: Record<string, string>) => string;
}

const I18nContext = createContext<I18nContextType | null>(null);

export function I18nProvider({ children }: { children: ReactNode }) {
  const [language, setLanguageState] = useState<Language>(() => {
    const stored = localStorage.getItem(STORAGE_KEY) as Language | null;
    return stored && (stored === "en" || stored === "de") ? stored : "en";
  });

  const setLanguage = (lang: Language) => {
    localStorage.setItem(STORAGE_KEY, lang);
    setLanguageState(lang);
  };

  const t = (key: string, replacements?: Record<string, string>) => {
    const keys = key.split(".");
    let value: unknown = translations[language];
    for (const k of keys) {
      if (value && typeof value === "object" && k in value) {
        value = (value as Record<string, unknown>)[k];
      } else {
        return key;
      }
    }
    if (typeof value !== "string") return key;
    let text = value;
    if (replacements) {
      for (const [rk, rv] of Object.entries(replacements)) {
        text = text.replace(new RegExp(`\\{${rk}\\}`, "g"), rv);
      }
    }
    return text;
  };

  return (
    <I18nContext.Provider value={{ language, setLanguage, t }}>
      {children}
    </I18nContext.Provider>
  );
}

export function useI18n() {
  const ctx = useContext(I18nContext);
  if (!ctx) throw new Error("useI18n must be used within I18nProvider");
  return ctx;
}
