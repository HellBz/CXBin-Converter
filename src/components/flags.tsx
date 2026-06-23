interface FlagProps {
  className?: string;
}

export function UkFlag({ className = "h-5 w-5" }: FlagProps) {
  return (
    <svg viewBox="0 0 60 30" className={className} aria-label="English">
      <clipPath id="s">
        <path d="M0,0 v30 h60 v-30 z" />
      </clipPath>
      <clipPath id="t" clipPath="url(#s)">
        <path d="M30,15 h30 v15 z v-15 h-30 z h-30 v15 z v-15 h30 z" />
      </clipPath>
      <g clipPath="url(#s)">
        <rect width="60" height="30" fill="#012169" />
        <path d="M0,0 L60,30 M60,0 L0,30" stroke="#fff" strokeWidth="6" />
        <path d="M0,0 L60,30 M60,0 L0,30" stroke="#C8102E" strokeWidth="4" clipPath="url(#t)" />
        <path d="M30,0 v30 M0,15 h60" stroke="#fff" strokeWidth="10" />
        <path d="M30,0 v30 M0,15 h60" stroke="#C8102E" strokeWidth="6" />
      </g>
    </svg>
  );
}

export function DeFlag({ className = "h-5 w-5" }: FlagProps) {
  return (
    <svg viewBox="0 0 5 3" className={className} aria-label="Deutsch">
      <rect width="5" height="1" y="0" fill="#000" />
      <rect width="5" height="1" y="1" fill="#D00" />
      <rect width="5" height="1" y="2" fill="#FFCE00" />
    </svg>
  );
}

export function FrFlag({ className = "h-5 w-5" }: FlagProps) {
  return (
    <svg viewBox="0 0 3 2" className={className} aria-label="Français">
      <rect width="1" height="2" x="0" fill="#002395" />
      <rect width="1" height="2" x="1" fill="#fff" />
      <rect width="1" height="2" x="2" fill="#ED2939" />
    </svg>
  );
}

export function EsFlag({ className = "h-5 w-5" }: FlagProps) {
  return (
    <svg viewBox="0 0 3 2" className={className} aria-label="Español">
      <rect width="3" height="0.5" y="0" fill="#AA151B" />
      <rect width="3" height="1" y="0.5" fill="#F1BF00" />
      <rect width="3" height="0.5" y="1.5" fill="#AA151B" />
    </svg>
  );
}

export function CnFlag({ className = "h-5 w-5" }: FlagProps) {
  return (
    <svg viewBox="0 0 3 2" className={className} aria-label="中文">
      <rect width="3" height="2" fill="#DE2910" />
      <g fill="#FFDE00">
        <path d="M0.6,0.35 l0.15,0.45 l-0.4,-0.28 h0.5 l-0.4,0.28 z" />
      </g>
    </svg>
  );
}

export function JpFlag({ className = "h-5 w-5" }: FlagProps) {
  return (
    <svg viewBox="0 0 3 2" className={className} aria-label="日本語">
      <rect width="3" height="2" fill="#fff" />
      <circle cx="1.5" cy="1" r="0.6" fill="#BC002D" />
    </svg>
  );
}
